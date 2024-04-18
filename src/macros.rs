macro_rules! cfg_ftp {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ftp")]
            $item
        )*
    }
}

macro_rules! cfg_smb {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "smb")]
            $item
        )*
    }
}

macro_rules! cfg_ssh {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ssh")]
            $item
        )*
    }
}
