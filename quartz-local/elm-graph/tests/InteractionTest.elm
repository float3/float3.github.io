module InteractionTest exposing (suite)

{-| The mouse: dragging a node, panning the picture, zooming it, and the one
click that is not a click.

This is the half of the graph a screenshot cannot show and a hidden tab cannot
be made to perform, so it is the half that is worth writing down. Elm will not
let a test look inside a `Cmd`, so where a message's whole job is to send one
-- following a node, say -- what is checked is the state that decides whether
it is sent at all.

-}

import Array
import Expect exposing (FloatingPointTolerance(..))
import Main
import Test exposing (Test, describe, test)


page : String -> List String -> Main.Page
page id links =
    { id = id, title = id, links = links, tags = [] }


{-| Two pages, one link, in a box 200 by 100 whose corner is at (50, 20).
-}
model : Main.Model
model =
    Main.build
        { flags
            | pages = [ page "one" [ "two" ], page "two" [] ]
            , slug = "one"
            , box = { width = 200, height = 100, left = 50, top = 20 }
        }


flags : Main.Flags
flags =
    Main.flags0


at : String -> Main.Model -> Main.Node
at id graph =
    graph.nodes
        |> Array.toList
        |> List.filter (\node -> node.id == id)
        |> List.head
        |> Maybe.withDefault
            { id = "", label = "", kind = Main.PageNode, degree = 0, x = 0, y = 0, vx = 0, vy = 0, pinned = False }


run : List Main.Msg -> Main.Model -> Main.Model
run messages graph =
    List.foldl (\message current -> Tuple.first (Main.update message current)) graph messages


{-| The middle of the box, in window coordinates: its corner plus half of it.
-}
middle : Main.Point
middle =
    { x = 50 + 100, y = 20 + 50 }


suite : Test
suite =
    describe "the mouse"
        [ describe "reading where it is"
            [ test "puts the middle of the box at the middle of the drawing" <|
                \_ ->
                    Main.where_ model middle
                        |> Expect.equal { x = 0, y = 0 }
            , test "measures from the box's corner, not the window's" <|
                \_ ->
                    Main.where_ model { x = 50, y = 20 }
                        |> Expect.equal { x = -100, y = -50 }
            , test "counts a unit as a pixel divided by the zoom" <|
                \_ ->
                    let
                        closer =
                            { model | camera = { zoom = 2, x = 0, y = 0 } }
                    in
                    Main.where_ closer { x = middle.x + 50, y = middle.y }
                        |> Expect.equal { x = 25, y = 0 }
            ]
        , describe "dragging a node"
            [ test "picks it up without moving it" <|
                \_ ->
                    let
                        before =
                            at "one" model

                        after =
                            at "one" (run [ Main.Grabbed "one" middle ] model)
                    in
                    ( after.x, after.y ) |> Expect.equal ( before.x, before.y )
            , test "pins the one it picked up and nothing else" <|
                \_ ->
                    let
                        held =
                            run [ Main.Grabbed "one" middle ] model
                    in
                    ( (at "one" held).pinned, (at "two" held).pinned )
                        |> Expect.equal ( True, False )
            , test "carries it by the distance the mouse travelled" <|
                \_ ->
                    let
                        before =
                            at "one" model

                        after =
                            at "one"
                                (run
                                    [ Main.Grabbed "one" middle
                                    , Main.Moved { x = middle.x + 30, y = middle.y - 10 }
                                    ]
                                    model
                                )
                    in
                    ( after.x - before.x, after.y - before.y )
                        |> Expect.equal ( 30, -10 )
            , test "carries it half as far when the picture is twice as close" <|
                \_ ->
                    let
                        closer =
                            { model | camera = { zoom = 2, x = 0, y = 0 } }

                        before =
                            at "one" closer

                        after =
                            at "one"
                                (run
                                    [ Main.Grabbed "one" middle
                                    , Main.Moved { x = middle.x + 30, y = middle.y }
                                    ]
                                    closer
                                )
                    in
                    after.x - before.x |> Expect.within (Absolute 1.0e-9) 15
            , test "leaves the others to the forces" <|
                \_ ->
                    let
                        dragged =
                            run
                                [ Main.Grabbed "one" middle
                                , Main.Moved { x = middle.x + 30, y = middle.y }
                                ]
                                model
                    in
                    (at "two" dragged).pinned |> Expect.equal False
            , test "gives the layout its energy back, so the rest gets out of the way" <|
                \_ ->
                    (run [ Main.Grabbed "one" middle ] { model | alpha = 0 }).alpha
                        |> Expect.within (Absolute 1.0e-9) 0.3
            , test "lets go of it when the mouse does" <|
                \_ ->
                    let
                        after =
                            run
                                [ Main.Grabbed "one" middle
                                , Main.Moved { x = middle.x + 30, y = middle.y }
                                , Main.Released
                                ]
                                model
                    in
                    ( (at "one" after).pinned, after.drag == Main.Still )
                        |> Expect.equal ( False, True )
            , test "does not let a node that was dragged wander off the screen with its own speed" <|
                \_ ->
                    -- A pinned node's velocity is thrown away every tick, so
                    -- letting go of one leaves it where it was put.
                    let
                        held =
                            run
                                [ Main.Grabbed "one" middle
                                , Main.Moved { x = middle.x + 30, y = middle.y }
                                , Main.Tick
                                ]
                                model
                    in
                    ( (at "one" held).vx, (at "one" held).vy ) |> Expect.equal ( 0, 0 )
            ]
        , describe "the click at the end of a drag"
            [ test "is not a link being followed" <|
                \_ ->
                    let
                        after =
                            run
                                [ Main.Grabbed "one" middle
                                , Main.Moved { x = middle.x + 30, y = middle.y }
                                , Main.Released
                                ]
                                model
                    in
                    after.dragged |> Expect.equal True
            , test "is a link being followed when the mouse never moved" <|
                \_ ->
                    let
                        after =
                            run [ Main.Grabbed "one" middle, Main.Released ] model
                    in
                    after.dragged |> Expect.equal False
            , test "is swallowed once and only once" <|
                \_ ->
                    let
                        after =
                            run
                                [ Main.Grabbed "one" middle
                                , Main.Moved { x = middle.x + 30, y = middle.y }
                                , Main.Released
                                , Main.Follow "one"
                                ]
                                model
                    in
                    after.dragged |> Expect.equal False
            ]
        , describe "panning"
            [ test "moves the picture with the hand" <|
                \_ ->
                    let
                        after =
                            run
                                [ Main.Panned middle
                                , Main.Moved { x = middle.x + 40, y = middle.y + 10 }
                                ]
                                model
                    in
                    ( after.camera.x, after.camera.y ) |> Expect.equal ( -40, -10 )
            , test "moves it half as far when the picture is twice as close" <|
                \_ ->
                    let
                        closer =
                            { model | camera = { zoom = 2, x = 0, y = 0 } }

                        after =
                            run
                                [ Main.Panned middle
                                , Main.Moved { x = middle.x + 40, y = middle.y }
                                ]
                                closer
                    in
                    after.camera.x |> Expect.within (Absolute 1.0e-9) -20
            , test "leaves the nodes exactly where they were" <|
                \_ ->
                    let
                        after =
                            run
                                [ Main.Panned middle
                                , Main.Moved { x = middle.x + 40, y = middle.y }
                                ]
                                model
                    in
                    ( (at "one" after).x, (at "one" after).y )
                        |> Expect.equal ( (at "one" model).x, (at "one" model).y )
            ]
        , describe "zooming"
            [ test "keeps whatever is under the pointer under the pointer" <|
                \_ ->
                    let
                        pointer =
                            { x = middle.x + 60, y = middle.y - 20 }

                        before =
                            Main.where_ model pointer

                        after =
                            run [ Main.Wheeled -250 pointer ] model
                    in
                    Main.where_ after pointer
                        |> Expect.all
                            [ .x >> Expect.within (Absolute 1.0e-9) before.x
                            , .y >> Expect.within (Absolute 1.0e-9) before.y
                            ]
            , test "goes in when the wheel goes up" <|
                \_ ->
                    (run [ Main.Wheeled -500 middle ] model).camera.zoom
                        |> Expect.within (Absolute 1.0e-9) 2
            , test "goes out when the wheel goes down" <|
                \_ ->
                    (run [ Main.Wheeled 500 middle ] model).camera.zoom
                        |> Expect.within (Absolute 1.0e-9) 0.5
            , test "stops at four times in" <|
                \_ ->
                    (run (List.repeat 10 (Main.Wheeled -500 middle)) model).camera.zoom
                        |> Expect.within (Absolute 1.0e-9) 4
            , test "stops at a quarter out" <|
                \_ ->
                    (run (List.repeat 10 (Main.Wheeled 500 middle)) model).camera.zoom
                        |> Expect.within (Absolute 1.0e-9) 0.25
            ]
        , describe "the labels"
            [ test "are hidden while the whole graph is in view" <|
                \_ ->
                    Main.labelOpacity { zoom = 1, x = 0, y = 0 }
                        |> Expect.within (Absolute 1.0e-9) 0
            , test "fade in as the reader zooms into a handful of nodes" <|
                \_ ->
                    Main.labelOpacity { zoom = 2.875, x = 0, y = 0 }
                        |> Expect.within (Absolute 1.0e-9) 0.5
            , test "never go past opaque, however far in the reader goes" <|
                \_ ->
                    Main.labelOpacity { zoom = 100, x = 0, y = 0 }
                        |> Expect.within (Absolute 1.0e-9) 1
            ]
        , describe "what the box is showing"
            [ test "is the whole drawing at rest" <|
                \_ ->
                    Main.viewBox model |> Expect.equal "-100 -50 200 100"
            , test "is half of it, around the camera, twice as close" <|
                \_ ->
                    Main.viewBox { model | camera = { zoom = 2, x = 10, y = 5 } }
                        |> Expect.equal "-40 -20 100 50"
            ]
        , describe "being told to stop"
            [ test "spends the alpha, which is what asks for the next frame" <|
                \_ ->
                    (run [ Main.Halted ] { model | alpha = 1 }).alpha
                        |> Expect.equal 0
            ]
        ]
