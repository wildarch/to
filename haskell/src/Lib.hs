{-# LANGUAGE DeriveGeneric #-}
module Lib (Settings(..), getSettings, saveSettings) where

import           Data.Aeson
import           Data.Aeson.Encode.Pretty
import           Data.ByteString.Lazy     as B
import           GHC.Generics
import           System.Directory
import           System.FilePath

confPath :: FilePath -> FilePath
confPath home = home </> ".config/to"

data Settings = Settings {directories :: [FilePath]} deriving Generic
instance ToJSON Settings
instance FromJSON Settings

getSettings :: IO (Either String Settings)
getSettings = do
    home  <- getHomeDirectory
    let path = confPath home
    exists <- doesFileExist path
    if exists then
      do
         content <- B.readFile path
         case decode content of
           Just s  -> return (Right s)
           Nothing -> return $ Left $ "Error: " ++ path ++ " is not valid"
    else
      do
        let settings = Settings []
        saveSettings settings
        return (Right settings)

saveSettings :: Settings -> IO ()
saveSettings settings = do
    home  <- getHomeDirectory
    let path = confPath home
    let content = encodePretty settings
    B.writeFile path content
