module Main exposing (..)

-- Press buttons to increment and decrement a counter.
--
-- Read how it works:
--   https://guide.elm-lang.org/architecture/buttons.html
--


import Browser
import Html exposing (Html, button, div, text, input)
import Html.Events exposing (onClick)
import Html.Attributes exposing (type_, value)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
  { value : Int
  , result: String
  }


init : Model
init =
  Model 0 ""



-- UPDATE


type Msg
  = Increment
  | Decrement
  | InvokeUpdate


update : Msg -> Model -> Model
update msg model =
  case msg of
    Increment ->
      Model (model.value + 1) ""

    Decrement ->
      Model (model.value - 1) ""

    InvokeUpdate -> Model model.value "result"

-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , input [type_ "text", value (String.fromInt model.value) ] []
    , button [ onClick Increment ] [ text "+" ]
    , button [ onClick InvokeUpdate ] [ text "Update" ]
    , div [] [ text ("Result: " ++ model.result) ]
    ]
