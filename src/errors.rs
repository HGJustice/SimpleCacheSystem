
    #[derive(Debug)]
    pub enum CacheSystemError {
        CacheFull,
        KeyNotFound, 
        InvalidValue,
        FIFOError,
        LRUError,
        SerializationError,
        FailedToInsertData
    }

    #[derive(Debug)]
    pub enum CacheDataError {
        InvalidData,
    }

    #[derive(Debug)]
    pub enum SerializeError{
        JsonError, 
        BinaryError, 
        DeserializeError,
        InvalidFormat
    }
