use std::collections::HashMap;
use std::str;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::DeserializeOwned;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead, Payload};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::Rng;

use crate::error::DataHandleError;

/// A configuration struct with optional settings and/or password-encryptable data.
/// 
/// T needs to implement serde serialize and deserialize
#[derive(Debug, Serialize, Deserialize)]
pub struct Config<T>
{
    pub settings: Option<T>,
    pub data: Option<LockedData>,
}

impl<'de, T> Config<T>
where T: Serialize + DeserializeOwned
{
    /// Serialize the configuration as a string of YAML.
    pub fn to_yaml(&self) -> Result<String, DataHandleError>
    {
        Ok(serde_yaml::to_string(&self)?)
    }

    /// Deserialize the configuration from a string of YAML.
    pub fn from_yaml(buffer: &str) -> Result<Self, DataHandleError>
    {
        Ok(serde_yaml::from_str(buffer)?)
    }
}

impl<T> Config<T>
{
    /// Create a new Config.
    pub fn new(settings: Option<T>, data: Option<LockedData>) -> Config<T>
    {
        Config { settings, data }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
/// A structure used for protecting data through encryption.
/// 
/// The purpose of this struct is to lock up a `data` struct using a password, after which the blobs will be encrypted using AES256GCM.
/// 
/// Argon2 will be used for password verification, AES256GCM will be used to encrypt the data and the key.
pub struct LockedData
{
    verification_hash: String,
    salt: String,
    #[serde(deserialize_with = "des_string_as_key")]
    #[serde(serialize_with = "ser_key_as_string")]
    key: Vec<u8>,
    data: Data,
}

/// yaml arrays are written multiline in serde, the conversion to string makes it a single line
pub fn ser_key_as_string<S>(key: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let key_as_string = String::from_utf8(key.to_owned()).map_err(|_| serde::ser::Error::custom("Vec<u8> to string error"))?;
    s.serialize_str(&key_as_string)
}

/// Deserialize the key string back into a Vec<u8> 
pub fn des_string_as_key<'de, D>(d: D) -> Result<Vec<u8>, D::Error> where D: Deserializer<'de>
{
    let key_as_string = String::deserialize(d)?;
    Ok(key_as_string.as_bytes().to_owned())
}

impl LockedData
{
    /// Creates a new LockData which holds variables used for encrypting and decrypting data.
    /// 
    /// # Examples
    /// ```
    /// let mut data_lock = LockedData::new("password")?;
    /// let mut data = data_lock.unlock("password")?;
    /// 
    /// data_lock.lock("password", data)?;
    /// ```
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    pub fn new(password: &str) -> Result<LockedData, DataHandleError>
    {
        // Generate a new key for AES256GCM
        let mut key = vec![0u8;32];
        rand::rngs::OsRng.fill(key.as_mut_slice());

        // Generate a salt for the first argon2 hash
        let salt = SaltString::generate(rand::rngs::OsRng).as_str().to_owned();
        let hashed_password = ArgonHelper::argon2_hash(password, &salt)?;

        let verification_salt = SaltString::generate(rand::rngs::OsRng).as_str().to_owned();
        let verification_hash = ArgonHelper::argon2_phc(&hashed_password.to_string(), &verification_salt)?.to_string();

        let data = Data::new();

        let decoded_hash = base64::decode(hashed_password)?;
        let key = AesHelper::encrypt_with_key_to_b64(&decoded_hash, key.as_ref())?;

        let key = key.as_bytes().to_owned();
        Ok(LockedData {verification_hash, salt, key, data})
    }


    /// Decrypt a `Data` structure using a password and return it on success.
    /// 
    /// The password will be verified using a Argon2 phc string,
    /// after which the blobs field in data will be decrypted using AES256GCM with the saved key.
    /// 
    /// # Examples
    /// ```
    /// let mut data_lock = LockedData::new("password")?;
    /// let mut data = data_lock.unlock("password")?;
    /// ```
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    /// 
    /// [base64](https://docs.rs/base64/0.13.0/base64/enum.DecodeError.html)
    /// 
    /// [FromUtf8](https://doc.rust-lang.org/std/string/struct.FromUtf8Error.html)
    /// 
    /// [Aes-GCM](https://docs.rs/aes-gcm/0.9.4/aes_gcm/struct.Error.html)
    pub fn unlock(&mut self, password: &str) -> Result<Data, DataHandleError>
    {
        let hash = ArgonHelper::verify_password(password, &self.salt, &self.verification_hash)?;

        // The hash is b64 encoded so we decode it here to make it 32 bits long to use as key for AES256GCM
        let decoded_hash = base64::decode(hash)?;

        // Decrypt the key used for the blobs with AES256GCM using the decoded_hash
        let key = AesHelper::decrypt_with_key_from_b64(&decoded_hash, &self.key)?;

        let mut data = self.data.clone();
        for (_index, data) in data.map.iter_mut()
        {
            let decrypted = AesHelper::decrypt_with_key_from_b64(&key, data.blob.as_bytes())?;
            data.blob = String::from_utf8(decrypted)?;
        }
        Ok(data)
    }


    /// Encrypt a `Data` structure using a password.
    /// 
    /// The password will be verified using a Argon2 phc string,
    /// after which the blobs field in data will be encrypted using AES256GCM with a new random key.
    /// The key will be encrypted using the password after the encryption of the `data` structure.
    /// 
    /// # Examples
    /// ```
    /// let mut data_lock = LockedData::new("password")?;
    /// let mut data = data_lock.unlock("password")?;
    /// 
    /// data_lock.lock("password", data)?;
    /// ```
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    /// 
    /// [base64](https://docs.rs/base64/0.13.0/base64/enum.DecodeError.html)
    /// 
    /// [Aes-GCM](https://docs.rs/aes-gcm/0.9.4/aes_gcm/struct.Error.html)
    pub fn lock(&mut self, password: &str, mut data: Data)  -> Result<(), DataHandleError>
    {
        let hash = ArgonHelper::verify_password(password, &self.salt, &self.verification_hash)?;

        // Create a new random key
        let mut key = vec![0u8;32];
        rand::rngs::OsRng.fill(key.as_mut_slice());

        for (_index, data) in data.map.iter_mut()
        {
            data.blob = AesHelper::encrypt_with_key_to_b64(&key, data.blob.as_bytes())?;
        }
        // The hash is b64 encoded so we decode it here to make it 32 bits long to use as key for AES256GCM
        let decoded_hash = base64::decode(hash)?;
        // Encrypt the key used for encrypting the blobs so we can store it safely
        let key = AesHelper::encrypt_with_key_to_b64(&decoded_hash, key.as_ref())?;

        self.key = key.as_bytes().to_owned();
        self.data = data;
        Ok(())
    }

    /// Change the current password.
    /// 
    /// 
    pub fn change_password(&mut self, old_password: &str, new_password: &str) -> Result<(), DataHandleError>
    {
        let hash = ArgonHelper::verify_password(old_password, &self.salt, &self.verification_hash)?;

        // The hash is b64 encoded so we decode it here to make it 32 bits long to use as key for AES256GCM
        let decoded_hash = base64::decode(hash)?;

        // Decrypt the key used for the blobs with AES256GCM using the decoded_hash
        let decrypted_key = AesHelper::decrypt_with_key_from_b64(&decoded_hash, &self.key)?;

        // Generate a salt for the first argon2 hash
        let salt = SaltString::generate(rand::rngs::OsRng).as_str().to_owned();
        let hashed_password = ArgonHelper::argon2_hash(new_password, &salt)?;

        let verification_salt = SaltString::generate(rand::rngs::OsRng).as_str().to_owned();
        let verification_hash = ArgonHelper::argon2_phc(&hashed_password.to_string(), &verification_salt)?.to_string();

        let decoded_hash = base64::decode(hashed_password)?;
        let encrypted_key = AesHelper::encrypt_with_key_to_b64(&decoded_hash, decrypted_key.as_ref())?;

        self.key = encrypted_key.as_bytes().to_owned();
        self.salt = salt;
        self.verification_hash = verification_hash;
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
/// Wrapper around a `HashMap<String, DataInformation>`.
pub struct Data
{
    map: HashMap<String, DataInformation>,
}

impl Data
{
    /// Create a new `Data` with empty Hashmap.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut data = Data::new();
    /// 
    /// data.generate(String::from("website"), None, 8, SymbolLevel::Symbols)?;
    /// data.insert(String::from("other_website"), DataInformation::new(None, None, String::from("password")))?;
    /// ``` 
    pub fn new() -> Data
    {
        let map= HashMap::new();
        Data { map }
    }


    /// Insert a Name - DataInformation with a randomly generated password into the map.
    ///
    /// The blob will be a randomly generated password based on the constraints password_length and symbol_level.
    ///
    /// The hint will be "Randomly generated password".
    /// # Examples
    /// ```
    /// let mut data = Data::new();
    /// data.generate(String::from)
    /// ```
    pub fn generate(&mut self, index: String, comment: Option<String>, password_length: u8, symbol_level: SymbolLevel) -> Result<(), DataHandleError>
    {
        let password = Data::generate_password(password_length, symbol_level);

        let hint = Some("Randomly generated password".to_string());

        let information = DataInformation { hint, comment, blob: password };

        self.insert(index, information);
        Ok(())
    }


    /// Generate a random ascii password constrained by the length and symbol level parameters.
    ///
    /// There are 4 levels defined in the [`SymbolLevel`] enum:
    /// - only lowercase
    /// - upper and lowercase
    /// - upper, lowercase and numbers
    /// - upper, lowercase, numbers and symbols
    ///
    /// # Examples
    /// ```
    /// let lowercase_5_letter_password = Data::generate_password(5, SymbolLevel::LowercaseAscii);
    /// let symbols_20_letter_password = Data::generate_password(20, SymbolLevel::Symbols);
    /// ```
    pub fn generate_password(password_length: u8, symbol_level: SymbolLevel) -> String
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


    /// Insert a Name - DataInformation pair into the map.
    ///
    /// This is a wrapper around HashMap's [`insert`](https://doc.rust-lang.org/nightly/std/collections/hash_map/struct.HashMap.html#method.insert).
    ///
    /// # Example
    /// 
    /// ```
    /// let mut data = Data::new();
    /// let information = DataInformation::new(None, None, String::from("some_password"));
    /// assert_eq!(None, data.insert(String::from("Website"), information));
    ///
    /// let new_information = DataInformation::new(None, None, String::from("new_password"));
    /// let result = Some(DataInformation { hint: None, comment:None, blob: String::from("some_password")})
    /// assert_eq!(result, data.insert(String::from("Website"), new_information));
    /// ```
    pub fn insert(&mut self, name: String, information: DataInformation) -> Option<DataInformation>
    {
        self.map.insert(name, information)
    }


    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// This is a wrapper around HashMap's [`remove`](https://doc.rust-lang.org/nightly/std/collections/hash_map/struct.HashMap.html#method.remove).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut data = Data::new();
    /// let information = DataInformation::new(None, None, String::from("some_password"));
    /// data.insert(String::from("Website"), information);
    ///
    /// assert_eq!(data.remove("Website"), Some(DataInformation { hint: None, comment:None, blob: String::from("some_password")}));
    /// assert_eq!(data.remove("Website"), None);
    /// ```
    pub fn remove(&mut self, name: &str) -> Option<DataInformation>
    {
        self.map.remove(name)
    }


    /// Returns a reference to the value corresponding to the key.
    ///
    /// This is a wrapper around HashMap's [`get`](https://doc.rust-lang.org/nightly/std/collections/hash_map/struct.HashMap.html#method.get).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut data = Data::new();
    /// let information = DataInformation::new(None, None, String::from("some_password"));
    /// data.insert(String::from("Website"), information);
    ///
    /// assert_eq!(data.get("Website"), Some(DataInformation { hint: None, comment:None, blob: String::from("some_password")}));
    /// ```
    pub fn get(&self, name: &str) -> Option<&DataInformation>
    {
        self.map.get(name)
    }

}

/// Holds the information for the data struct.
///
/// `hint` and `comment` are optional, descriptive fields, and will stay as plaintext in the configuration file.
/// The `blob` field will be encrypted using Aes256Gcm and converted to a b64 string for conversion purposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataInformation
{
    pub hint: Option<String>,
    pub comment: Option<String>,
    pub blob: String,
}

impl DataInformation
{
    /// Construct a new DataInformation.
    /// 
    /// # Examples
    /// ```
    /// let data = DataInformation::new(None, None, String::from("Maybe_Some_password"));
    /// ```
    pub fn new(hint: Option<String>, comment: Option<String>, blob: String) -> DataInformation
    {
        DataInformation { hint, comment, blob}
    }
}

#[cfg(test)]
mod configtests
{
    use super::*;

    #[test]
    fn lock_data()
    {
        let mut data = Data::new();
        let information = DataInformation::new(None, None, String::from("some_password"));
        data.insert(String::from("Website"), information);
    }

    #[test]
    fn test_insert_new_data()
    {
        let mut data = Data::new();
        let information = DataInformation::new(None, None, String::from("some_password"));
        assert_eq!(None, data.insert(String::from("Website"), information));
        let new_information = DataInformation::new(None, None, String::from("new_password"));
        let result = Some(DataInformation { hint: None, comment:None, blob: String::from("some_password")});
        assert_eq!(result, data.insert(String::from("Website"), new_information));
    }

    #[test]
    fn test_remove_data()
    {
        let mut data = Data::new();
        let information = DataInformation::new(None, None, String::from("some_password"));
        data.insert(String::from("Website"), information);
    
        assert_eq!(data.remove("Website"), Some(DataInformation { hint: None, comment:None, blob: String::from("some_password")}));
        assert_eq!(data.remove("Website"), None);
    }
}

/// A helper struct for Aes2Gcm encryption and decryption.
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

/// A helper struct for Argon2 encryption and verification.
struct ArgonHelper{}

impl ArgonHelper
{ 
    /// Return the PHC string from the result of the argon2 password hash algorithem.    
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    fn argon2_phc<'a>(password: &str, salt: &'a str) -> Result<PasswordHash<'a>, DataHandleError>
    {
        let argon2 = Argon2::default();
        let hash = argon2.hash_password_simple(password.as_ref(), salt)?;
        Ok(hash)
    }    

    /// Return the hash string from the phc after hashing the password.
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    fn argon2_hash<'a>(password: &str, salt: &str) -> Result<String, DataHandleError>
    {
        match ArgonHelper::argon2_phc(password, salt)?.hash
        {
            Some(output) => Ok(output.to_string()),
            None => Err(DataHandleError::LockedData("bad password".to_string()))
        }
    }

    /// Verify a password using a salt and phc string.
    /// # Errors
    /// [Argon / password_hash](https://docs.rs/password-hash/0.3.2/password_hash/errors/enum.Error.html)
    fn verify_password(password: &str, salt: &str, verification_hash: &str) -> Result<String, DataHandleError>
    {
        // initialise argon with default settings
        let argon = Argon2::default();
    
        let hash = ArgonHelper::argon2_hash(password, salt)?;

        let verification_hash = PasswordHash::new(verification_hash)?;
        argon.verify_password(hash.as_ref(), &verification_hash)?;
        Ok(hash)
    }
}

/// Symbollevel indicates what type of characters will be in a generated password.
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