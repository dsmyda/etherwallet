use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about="\nA for funsies ethereum wallet. \nThis implementation is BIP-39, BIP-32 and EIP-55 compliant.")]
enum Args {
    #[structopt(about="🚀 Create a wallet! Start from scratch or use an existing mnemonic.")]
    Create {

    },
    #[structopt(about="🤑 Send crypto an externally owned account (EOA).")]
    Send {
      address: String,
      amount: u64
    },
    #[structopt(about="😭 Find out how broke you are!")]
    Balance {

    }
}

fn main() {
  match Args::from_args() {
    Args::Create { } => {

    },
    Args::Send { address, amount } => {
        println!("{:?}", address);
        println!("{:?}", amount);
    }
    Args::Balance { } => {
    
    }
  }
}