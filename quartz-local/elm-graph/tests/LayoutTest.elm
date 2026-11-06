module LayoutTest exposing (suite)

{-| The graph, before anybody has touched it: which nodes are drawn, what they
are called, and where the forces put them.

These are the parts that were read off d3 and rewritten, so they are the parts
worth pinning down. A browser can show that the picture appears; only this can
say that a link pulls its ends to the distance it was asked for.

-}

import Array
import Expect exposing (FloatingPointTolerance(..))
import Main
import Set
import Test exposing (Test, describe, test)



-- A SMALL SITE
--
--   index  ->  blog/one  ->  blog/two
--   index  ->  missing         (a link to a page that is not there)
--   blog/one is tagged #things


page : String -> List String -> List String -> Main.Page
page id links tags =
    { id = id, title = String.toUpper id, links = links, tags = tags }


pages : List Main.Page
pages =
    [ page "index" [ "blog/one", "missing" ] []
    , page "blog/one" [ "blog/two" ] [ "things" ]
    , page "blog/two" [] []
    ]


{-| The flags a graph with nothing in it is built from, which is where every
fixture here starts.
-}
empty : Main.Flags
empty =
    Main.flags0


site : Main.Flags
site =
    { empty | pages = pages, slug = "index", showTags = True }


at : String -> Main.Model -> Maybe Main.Node
at id model =
    model.nodes |> Array.toList |> List.filter (\node -> node.id == id) |> List.head


distance : Main.Node -> Main.Node -> Float
distance one other =
    sqrt (((one.x - other.x) ^ 2) + ((one.y - other.y) ^ 2))


suite : Test
suite =
    describe "the graph"
        [ describe "its links"
            [ test "run from a page to the pages it links to" <|
                \_ ->
                    Main.allEdges site
                        |> List.member ( "index", "blog/one" )
                        |> Expect.equal True
            , test "leave out a link to a page that is not there" <|
                \_ ->
                    Main.allEdges site
                        |> List.member ( "index", "missing" )
                        |> Expect.equal False
            , test "include the tags a page carries" <|
                \_ ->
                    Main.allEdges site
                        |> List.member ( "blog/one", "tags/things" )
                        |> Expect.equal True
            , test "leave the tags out when the tags are turned off" <|
                \_ ->
                    Main.allEdges { site | showTags = False }
                        |> List.member ( "blog/one", "tags/things" )
                        |> Expect.equal False
            , test "leave out a tag that was asked to be left out" <|
                \_ ->
                    Main.allEdges { site | removeTags = [ "things" ] }
                        |> List.member ( "blog/one", "tags/things" )
                        |> Expect.equal False
            ]
        , describe "its neighbourhood"
            [ test "at no depth at all is the page and nothing else" <|
                \_ ->
                    Main.neighbourhood 0 "index" (Main.allEdges site)
                        |> Expect.equal (Set.fromList [ "index" ])
            , test "at one step is what the page touches" <|
                \_ ->
                    Main.neighbourhood 1 "index" (Main.allEdges site)
                        |> Expect.equal (Set.fromList [ "index", "blog/one" ])
            , test "at two steps is what those touch, in either direction" <|
                \_ ->
                    Main.neighbourhood 2 "index" (Main.allEdges site)
                        |> Expect.equal
                            (Set.fromList [ "index", "blog/one", "blog/two", "tags/things" ])
            , test "walks a link backwards as readily as forwards" <|
                \_ ->
                    Main.neighbourhood 1 "blog/two" (Main.allEdges site)
                        |> Expect.equal (Set.fromList [ "blog/two", "blog/one" ])
            , test "at a negative depth is the whole site" <|
                \_ ->
                    Main.neighbourhood -1 "index" (Main.allEdges site)
                        |> Set.size
                        |> Expect.equal 4
            ]
        , describe "its nodes"
            [ test "are the neighbourhood, one apiece" <|
                \_ ->
                    Main.build { site | depth = -1 }
                        |> .nodes
                        |> Array.length
                        |> Expect.equal 4
            , test "carry the page's title" <|
                \_ ->
                    Main.build { site | depth = 1 }
                        |> at "blog/one"
                        |> Maybe.map .label
                        |> Expect.equal (Just "BLOG/ONE")
            , test "fall back to the slug when a page has no title" <|
                \_ ->
                    Main.build { site | pages = [ page "index" [] [], { id = "blog/one", title = "", links = [ "index" ], tags = [] } ] }
                        |> at "blog/one"
                        |> Maybe.map .label
                        |> Expect.equal (Just "blog/one")
            , test "write a tag as a tag" <|
                \_ ->
                    Main.build { site | depth = -1 }
                        |> at "tags/things"
                        |> Maybe.map (\node -> ( node.label, node.kind ))
                        |> Expect.equal (Just ( "#things", Main.TagNode ))
            , test "have a radius of two plus the root of their links" <|
                \_ ->
                    Main.build { site | depth = -1 }
                        |> at "blog/one"
                        |> Maybe.map Main.radius
                        |> Maybe.withDefault 0
                        |> Expect.within (Absolute 1.0e-9) (2 + sqrt 3)
            ]
        , describe "where they start"
            [ test "is never two nodes in the same place" <|
                \_ ->
                    let
                        seeds =
                            List.range 0 99 |> List.map Main.phyllotaxis
                    in
                    seeds
                        |> Set.fromList
                        |> Set.size
                        |> Expect.equal (List.length seeds)
            , test "is the same spiral every time the page is opened" <|
                \_ ->
                    Main.phyllotaxis 7
                        |> Expect.equal (Main.phyllotaxis 7)
            ]
        , describe "where the forces put them"
            [ test "puts two linked pages about a link's length apart" <|
                \_ ->
                    let
                        laid =
                            Main.lay (Main.build { site | depth = 1, showTags = False })

                        apart =
                            Maybe.map2 distance (at "index" laid) (at "blog/one" laid)
                                |> Maybe.withDefault 0
                    in
                    Expect.all
                        [ \value -> Expect.greaterThan 20 value
                        , \value -> Expect.lessThan 45 value
                        ]
                        apart
            , test "is centred on the middle of the box" <|
                \_ ->
                    let
                        laid =
                            Main.lay (Main.build { site | depth = -1 })

                        middle =
                            laid.nodes
                                |> Array.toList
                                |> List.foldl (\node total -> total + node.x) 0
                    in
                    Expect.within (Absolute 1) 0 middle
            , test "leaves nothing at a number that is not one" <|
                \_ ->
                    Main.lay (Main.build { site | depth = -1 })
                        |> .nodes
                        |> Array.toList
                        |> List.all (\node -> not (isNaN node.x) && not (isNaN node.y))
                        |> Expect.equal True
            , test "stops moving once the alpha is spent" <|
                \_ ->
                    let
                        laid =
                            Main.lay (Main.build { site | depth = -1, reducedMotion = True })
                    in
                    Expect.within (Absolute 1.0e-12) 0 laid.alpha
            ]
        ]
