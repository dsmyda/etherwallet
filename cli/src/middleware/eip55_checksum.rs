use hex;
use sha3::{Digest, Keccak256};

/**
  Protect the integrity of the address against typing or reading mistakes.
  Important if you don't want to send funds to the wrong person ;)
  Ref: https://github.com/Ethereum/EIPs/blob/master/EIPS/eip-55.md
*/

pub fn verify(address: String) -> bool {
  // Reduce the address to just a lowercased 20-byte hex string
  let normalized_address = address.to_lowercase().replace("0x", "");
  let normalized_hash = hex::encode(
    Keccak256::new()
      .chain(normalized_address.as_bytes())
      .finalize(),
  );
  let mut checksummed_address = "".to_owned();
  let mut hash_iter = normalized_hash.chars();
  for hex_char in normalized_address.chars() {
    let hashed_character = hash_iter.next().unwrap();
    if "0123456789".contains(hex_char) {
      // Cannot uppercase a digit
      checksummed_address.push(hex_char);
    } else if "abcdef".contains(hex_char) {
      if hashed_character.to_digit(16).unwrap() > 7 {
        checksummed_address.push_str(&hex_char.to_uppercase().to_string());
      } else {
        checksummed_address.push(hex_char);
      }
    } else {
      panic!("Detected a non hex character in address {}.", address)
    }
  }
  let checksummed_address = "0x".to_owned() + &checksummed_address;
  return checksummed_address.eq(&address);
}

mod tests {}
