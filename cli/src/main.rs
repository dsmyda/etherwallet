mod middleware;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  about = "\nA for funsies ethereum wallet. \nThis implementation is BIP-39, BIP-32 and EIP-55 compliant."
)]
enum Args {
  #[structopt(about = "🚀 Create a wallet! Start from scratch or use an existing mnemonic.")]
  Create {},
  #[structopt(about = "🤑 Send ether to an externally owned account (EOA).")]
  Send { address: String, amount: u64 },
  #[structopt(about = "😭 Find out how broke you are!")]
  Balance {},
  #[structopt(about = "🔒 Lock your wallet to keep your funds secure.")]
  Lock {},
}

fn main() {
  match Args::from_args() {
    Args::Create {} => {}
    Args::Send { address, amount } => {
      if !middleware::eip55_checksum::verify(address) {
        println!("Checksum failed. Is there a typo in your address?");
      } else {
        println!("Funds sent!")
      }
      // web3::
      //let transport = web3::transports::Http::new("https://main-light.eth.linkpool.io").unwrap();
      // let web3 = web3::Web3::new(transport);
      // let hash = web3.web3().sha3(bytes: Bytes)
    }
    Args::Balance {} => {}
    Args::Lock {} => {
      println!("Locked!");
    }
  }
}
