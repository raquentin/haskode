{-# LANGUAGE DataKinds #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TemplateHaskell #-}

module Api where

import Data.Aeson (FromJSON, ToJSON)
import GHC.Generics (Generic)
import Servant
import Data.Text (Text)

data User = User
{
userId :: Int,
userEmail :: Text,
userPassword :: Text,
} deriving (Eq, Show, Generic)

data Question = Question
{
questionId :: Int
, questionTitle :: Text
, questionMd :: Text
} deriving (Eq, Show, Generic)

instance FromJSON Question
instance ToJSON Question


