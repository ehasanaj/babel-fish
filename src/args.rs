use clap::{command, Parser};

/// The Babel fish is small, yellow and leech-like, and probably the oddest thing in the Universe. It feeds on brainwave energy received not from its own carrier but from those around it. It absorbs all unconscious mental frequencies from this brainwave energy to nourish itself with. It then excretes into the mind of its carrier a telepathic matrix formed by combining the conscious thought frequencies with the nerve signals picked up from the speech centres of the brain which has supplied them. The practical upshot of all this is that if you stick a Babel fish in your ear you can instantly understand anything said to you in any form of language. The speech patterns you actually hear decode the brainwave matrix which has been fed into your mind by your Babel fish.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Destination language
    #[arg(short, long, default_value = "EN")]
    destination: String,

    /// Text to translate
    #[arg(short, long)]
    text: String,
}

impl Args {
    /// Get destination language
    pub fn destination(&self) -> &String {
        &self.destination
    }

    /// Get text to translate
    pub fn text(&self) -> &String {
        &self.text
    }
}
