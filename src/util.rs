#[macro_export]
macro_rules! block {
    ($e:expr, $dur:expr) => {
        loop {
            #[allow(unreachable_patterns)]
            match $e {
                Err(nb::Error::Other(e)) => {
                    #[allow(unreachable_code)]
                    break Err(e);
                }
                Err(nb::Error::WouldBlock) => {
                    embassy_time::Timer::after($dur).await;
                }
                Ok(x) => break Ok(x),
            }
        }
    };
}
