{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TypeOperators #-}

module Main (main) where

import Network.Wai
import Network.Wai.Handler.Warp
import Servant
import Api
import Handlers

app :: Application
app = serve (Proxy :: Proxy API) server

main :: IO()
main = run 8080 app
