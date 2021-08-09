use std::collections::HashMap;
use std::str::{self, Bytes};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{DeserializeOwned, Expected};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead, Payload};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rand::Rng;
use crate::error::DataHandleError;

/// Struct to hold the configuration from yaml
#[derive(Debug, Serialize, Deserialize)]
pub struct Config<T>
{
    pub settings: Option<T>,
    pub data: Option<Data>,

}

impl<'de, T> Config<T>
where T: Serialize + DeserializeOwned
{
    pub fn to_yaml(&mut self, password: &str) -> Result<String, DataHandleError>
    {
        if let Some(d) = &mut self.data
        {
            d.update_encryption()?;
            d.lock(password)?;
        }

        Ok(serde_yaml::to_string(&self)?)
    }

    pub fn from_yaml(buffer: &str) -> Result<Self, DataHandleError>
    {
        Ok(serde_yaml::from_str(buffer)?)
    }
}

impl<T> Config<T>
{
    pub fn new(settings: Option<T>, data: Option<Data>) -> Config<T>
    {
        Config { settings, data }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data
{
    #[serde(skip, default = "default_lock")]
    is_locked: bool,
    salt: String,
    #[serde(deserialize_with = "deserialize_str_to_vec", serialize_with = "serialize_vec_to_str")]
    key: Vec<u8>,
    pub data: HashMap<String, DataInformation>
}

fn deserialize_str_to_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    Ok(buf.as_bytes().to_owned())
}

fn serialize_vec_to_str<S>(key: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    let str = str::from_utf8(key).map_err(|_| serde::ser::Error::custom("invalid utf8"))?;
    s.serialize_str(&str)
}

fn default_lock() -> bool
{
    true
}

impl Data
{
    pub fn new() -> Data
    {
        let salt = SaltString::generate(rand::rngs::OsRng).as_str().to_owned();
        let key = vec![0u8;32];
        let data = HashMap::new();

        Data { is_locked: false, salt, key, data }
    }

    pub fn lock(&mut self, password: &str) -> Result<(), DataHandleError>
    {
        if self.is_locked
        {
            return Ok(())
        }

        let argon = Argon2::default();

        let salt = SaltString::generate(rand::rngs::OsRng);

        let hash = argon.hash_password_simple(password.as_ref(), salt.as_ref())?.hash.unwrap();

        self.key = AesHelper::encrypt_with_key_to_b64(hash.as_bytes(), self.key.as_ref())?.as_bytes().to_owned();

        self.salt = salt.as_str().to_owned();

        return Ok(())
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), DataHandleError>
    {
        if self.is_locked
        {
            let argon = Argon2::default();

            let salt = SaltString::new(&self.salt)?;
    
            let hash = argon.hash_password_simple(password.as_ref(), salt.as_ref())?.hash.unwrap();
    
            self.key = AesHelper::decrypt_with_key_from_b64(hash.as_bytes(), &self.key)?;

            self.is_locked = false;
        }
        return Ok(())
    }

    fn update_encryption(&mut self) -> Result<(), DataHandleError>
    {
        if self.is_locked
        {
            return Err(DataHandleError::LockedData("Can't update encryption, data is locked".to_owned()))
        }

        let mut new_key = vec![0u8;32];
        rand::rngs::OsRng.fill(new_key.as_mut_slice());

        for (_index, data) in self.data.iter_mut()
        {
            let blob = AesHelper::decrypt_with_key_from_b64(&self.key, data.blob.as_bytes())?;
            data.blob = AesHelper::encrypt_with_key_to_b64(&new_key, blob.as_ref())?;
        }
        self.key = new_key;
        Ok(())
    }

    pub fn generate(&mut self, index: String, comment: Option<String>, password_length: u8, symbol_level: SymbolLevel) -> Result<(), DataHandleError>
    {
        if self.is_locked
        {
            return Err(DataHandleError::LockedData("Can't generate a password, data is locked".to_owned()))
        }

        let password = generate_password(password_length, symbol_level);

        let b64_password = AesHelper::encrypt_with_key_to_b64(&self.key, password.as_ref())?;

        let hint = Some("Randomly generated password".to_string());

        let information = DataInformation { hint, comment, blob: b64_password };

        self.data.insert(index, information);
        Ok(())
    }

    pub fn insert(&mut self, index: String, blob: String, hint: Option<String>, comment: Option<String>) -> Result<(), DataHandleError>
    {
        if !self.data.contains_key(&index)
        {
            self.insert_or_update(index, blob, hint, comment)?;
        }
        Ok(())
    }

    pub fn insert_or_update(&mut self, index: String, blob: String, hint: Option<String>, comment: Option<String>) -> Result<(), DataHandleError>
    {
        if self.is_locked
        {
            return Err(DataHandleError::LockedData("Can't insert a password, data is locked".to_owned()))
        }

        let blob = AesHelper::encrypt_with_key_to_b64(&self.key, blob.as_ref())?;

        let information = DataInformation { hint, comment, blob: blob };

        self.data.insert(index, information);
        Ok(())
    }

    pub fn decrypt_blob(&self, index: &str) -> Option<Result<Vec<u8>, DataHandleError>>
    {
        if self.is_locked
        {
            return Some(Err(DataHandleError::LockedData("Can't decrypt blob, data is locked".to_owned())))
        }

        match self.data.get(index).map(|information| information.blob.clone())
        {   
            Some(b64_blob) => 
            {
                Some(AesHelper::decrypt_with_key_from_b64(&self.key, b64_blob.as_ref()))
            }
            None => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataInformation
{
    pub hint: Option<String>,
    pub comment: Option<String>,
    pub blob: String,
}

fn generate_password(password_length: u8, symbol_level: SymbolLevel) -> String
{
    let pool = b"abcdefghijklmnopqrstuwvxyzABCDEFGIJKLHMNOPQRSTUVWXYZ0123456789~!@#$%^&*_-+=`|\\(){}[]:;'<>,.?/";
    let mut generated_pass = String::new();
    let mut gen = rand::thread_rng();
    let symbol = symbol_level as usize;
    for _ in 0..password_length
    {
        let rng = gen.gen_range(0..symbol);
        generated_pass.push(pool[rng] as char);
    }
    generated_pass
}

struct AesHelper{}

impl AesHelper
{
    fn encrypt_with_key_to_b64<'msg, 'aad>(key: &[u8], plaintext: impl Into<Payload<'msg, 'aad>>) -> Result<String, DataHandleError>
    {
        let encrypted = AesHelper::encrypt_with_key(key, plaintext)?;
        Ok(base64::encode(encrypted))
    }

    fn encrypt_with_key<'msg, 'aad>(key: &[u8], plaintext: impl Into<Payload<'msg, 'aad>>) -> Result<Vec<u8>, DataHandleError>
    {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Ok(cipher.encrypt(Nonce::from_slice(&[0u8;12]), plaintext)?)
    }

    fn decrypt_with_key_from_b64<'msg, 'aad>(key: &[u8], encryptedtext: &[u8]) -> Result<Vec<u8>, DataHandleError>
    {
        let blob = base64::decode(encryptedtext).expect("remove me");
        AesHelper::decrypt_with_key(key, blob.as_ref())
    }

    fn decrypt_with_key<'msg, 'aad>(key: &[u8], encryptedtext: impl Into<Payload<'msg, 'aad>>) -> Result<Vec<u8>, DataHandleError>
    {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Ok(cipher.decrypt(Nonce::from_slice(&[0u8;12]), encryptedtext)?)
    }
}

pub enum SymbolLevel
{
    /// Use only lowercase ascii characters in the password
    LowercaseAscii = 26,
    /// Use upper and lowercase ascii characters in the password
    UppercaseAscii = 52,
    /// Use numbers and upper and lowercase ascii characters in the password
    Numbers = 62,
    /// Use numbers, symbols and upper and lowercase ascii characters in the password
    Symbols = 77,
}