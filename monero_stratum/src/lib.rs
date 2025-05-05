pub mod login;
pub mod submit;

#[cfg(test)]
mod test;

use log::trace;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_slice, to_string};
use snafu::{ResultExt, Snafu};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Debug, Snafu)]
pub enum Error {
    Connect { source: io::Error },
    Read { source: io::Error },
    Write { source: io::Error },

    Serialize { source: serde_json::Error },
    Deserialize { source: serde_json::Error },
}

pub trait Request<T, R> {
    async fn request(address: String, port: u16, request: T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned + Send,
    {
        trace!("Connecting to socket at {}:{}", address, port);
        let mut stream = TcpStream::connect(format!("{}:{}", address, port))
            .await
            .context(ConnectSnafu)?;

        trace!("Sending request");
        stream
            .write_all((to_string(&request).context(SerializeSnafu)? + "\n").as_bytes())
            .await
            .context(WriteSnafu)?;

        trace!("Receiving request");
        let mut buffer = vec![];
        BufReader::new(stream)
            .read_until(b'\n', &mut buffer)
            .await
            .context(ReadSnafu)?;

        let response: R = from_slice(&buffer).context(DeserializeSnafu)?;
        Ok(response)
    }
}
