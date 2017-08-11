module Test.Reporter.Highlightable exposing (Highlightable, fromDiff, fromLists, map)

import Diff exposing (Change(..))


type Highlightable a
    = Highlighted a
    | Plain a


fromLists : List a -> List a -> List (Highlightable a)
fromLists expected actual =
    -- TODO make sure this looks reasonable for multiline strings
    Diff.diff expected actual
        |> List.concatMap fromDiff


map : (a -> b) -> Highlightable a -> Highlightable b
map transform highlightable =
    case highlightable of
        Highlighted val ->
            Highlighted (transform val)

        Plain val ->
            Plain (transform val)


fromDiff : Change a -> List (Highlightable a)
fromDiff diff =
    case diff of
        Added char ->
            []

        Removed char ->
            [ Highlighted char ]

        NoChange char ->
            [ Plain char ]
