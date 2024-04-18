macro_rules! cfg_ftp {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ftp")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "ftp")))]
            $item
        )*
    }
}

macro_rules! cfg_smb {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "smb")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "smb")))]
            $item
        )*
    }
}

macro_rules! cfg_ssh {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "ssh")]
            #[cfg_attr(doc_cfg, doc(cfg(feature = "ssh")))]
            $item
        )*
    }
}
