module Main where

import           Control.Monad
import           Data.List
import           Lib
import           System.Directory
import           System.Environment
import           System.FilePath

main :: IO ()
main = do
    args <- getArgs
    settings <- getSettings
    currentDir <- getCurrentDirectory
    case settings of
      Left e -> putStrLn e
      Right settings ->
        let searchDirs = directories settings
        in
          case args of
            [] -> putStrLn "Error: no command given"
            ("add":dir:_) -> add dir settings
            ("add":_) -> add currentDir settings
            ("remove":index:_) -> remove index settings
            ("go":query:_) -> go query searchDirs
            ("list":query:_) -> list query searchDirs
            ("dirs":_) -> dirs searchDirs
            ("version":_) -> putStrLn "To: haskell version"
            (cmd:_) -> putStrLn $ "Error: invalid command '" ++ cmd  ++ "' given"

add :: FilePath -> Settings -> IO ()
add path settings = do
    isValid <- doesDirectoryExist path
    let settings' = if isValid
          then Settings $ path:(directories settings)
          else settings
    saveSettings settings'

remove :: String -> Settings -> IO ()
remove index settings = case reads index of
    [(i, "")] -> do
        let dirs = map snd $ filter (\(j,_) -> j /= i) $ zip [0..] (directories settings)
        saveSettings (Settings dirs)
    _ -> putStrLn "Error: index is not valid"

getSubPaths :: FilePath -> IO [FilePath]
getSubPaths path = do
    subDirs <- listDirectory path
    return $ map (\d -> path </> d) subDirs

getSubDirMatches :: String -> [FilePath] -> IO [FilePath]
getSubDirMatches query paths = do
    dirs <- mapM getSubPaths paths
    let match dir = do
          valid <- doesDirectoryExist dir
          return (valid && (query `isPrefixOf` (takeBaseName dir)))
    res <- filterM match $ concat dirs
    return (sort res)

go :: String -> [FilePath] -> IO ()
go query paths = do
    matches <- getSubDirMatches query paths
    case matches of
        []    -> putStrLn "Error: no results found"
        (m:_) -> putStrLn m

list :: String -> [FilePath] -> IO ()
list query paths = do
    matches <- getSubDirMatches query paths
    mapM_ (putStrLn . takeBaseName) matches

dirs :: [FilePath] -> IO ()
dirs [] = putStrLn "Error: no directories in settings file"
dirs ds = mapM_ (\(i, d) -> putStrLn $ "[" ++ (show i) ++ "] " ++ d) $ zip [0..] ds
