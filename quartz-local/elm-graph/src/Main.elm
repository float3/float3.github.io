port module Main exposing (..)

{-| The site graph, drawn in Elm.

Quartz's own graph component fetches d3 and pixi.js from a CDN -- a megabyte
and a half of somebody else's JavaScript, pulled from a third party on every
page view, to draw fifty circles -- and its script opens with `@ts-nocheck`
because none of it type-checks. This draws the same picture as SVG, with the
force simulation written here.

The layout is d3-force's on purpose: the same charge, link, centring and
collision forces, the same alpha decay, so the graph settles into the shape the
old one settled into. What is different is that nothing is random. d3 seeds
positions with `Math.random`; this uses the phyllotaxis spiral d3 falls back to
when a position is missing, so the same page draws the same graph every time it
is opened.

Colour is left to the stylesheet. The old script read six custom properties out
of `getComputedStyle` when it started, which is why it had to be told to redraw
when the theme changed. Here a node is a circle with a class on it and the CSS
says what that means in each theme.

Everything is exposed for one reason: the tests in `../tests` read it. Elm
builds an application from `main` outwards whatever the module line says, so
nothing extra is compiled into the page for it, and the alternative was a
twenty-name export list that would go stale the first time a test wanted one
more.

-}

import Array exposing (Array)
import Browser
import Browser.Events
import Dict exposing (Dict)
import Html exposing (Html)
import Html.Attributes
import Html.Events
import Json.Decode as Decode exposing (Decoder)
import Set exposing (Set)
import Svg exposing (Svg)
import Svg.Attributes as Attr
import Svg.Events



-- PORTS


{-| The id of a node the reader clicked. The page turns it into a URL and hands
that to Quartz's router, so that following a node is the same soft navigation
as following a link in the prose. The page is also the side that knows which
pages have been visited, so it is the side that writes this one down.
-}
port follow : String -> Cmd msg


{-| Said out loud rather than swallowed: flags this cannot read mean a graph
that cannot be drawn, and the reason belongs in the console where whoever
changed the shape of them will see it.
-}
port failed : String -> Cmd msg


{-| Where the container is and how big, watched by a `ResizeObserver` and the
page's scrolling on the other side.

Elm can hear the window resize but cannot measure a div, and this one lives in
a sidebar that changes width without the window doing anything. The corner
comes with it because a mouse event says where it is in the window and the
graph needs to know where that is in the box -- `getBoundingClientRect` is a
method, and an event decoder can only read properties.
-}
port resized : (Box -> msg) -> Sub msg


{-| Stop.

A soft navigation patches the sidebar back to the empty container the new page
came with -- the same element, with this app's drawing taken out of it -- and
the page starts another app in its place. Without this the old one would go on
running a simulation against a view nobody can see, for as long as its layout
took to settle.
-}
port halt : (() -> msg) -> Sub msg


{-| The container, in CSS pixels: how big it is, and where its top left corner
sits in the window.
-}
type alias Box =
    { width : Float, height : Float, left : Float, top : Float }


{-| A place, in whichever space the name around it says. -}
type alias Point =
    { x : Float, y : Float }


{-| What part of the drawing the box is showing.

`zoom` is how many pixels a unit of the layout takes up, and the rest is where
the middle of the box is over it. All three are read straight into the
`viewBox`, so panning and zooming move the picture without touching a single
node.
-}
type alias Camera =
    { zoom : Float, x : Float, y : Float }


{-| Looking at the middle of the layout, one pixel to the unit. -}
resting : Camera
resting =
    { zoom = 1, x = 0, y = 0 }


{-| d3's own limits, and for the same reason: further out than a quarter and
the graph is a smudge, closer in than four and it is one node.
-}
closest : Float
closest =
    4


furthest : Float
furthest =
    0.25


{-| What the mouse is doing, if anything.

`moved` is what tells a drag from a click. A node is a link, so letting go of
one after hauling it across the box would otherwise navigate, which is not what
the hand that hauled it meant.
-}
type Drag
    = Still
    | Holding { id : String, grab : Point, moved : Bool }
    | Panning { from : Point, moved : Bool }



-- FLAGS


type alias Flags =
    { slug : String
    , base : String
    , box : Box
    , depth : Int
    , showTags : Bool
    , removeTags : List String
    , repelForce : Float
    , centerForce : Float
    , linkDistance : Float
    , focusOnHover : Bool
    , enableRadial : Bool
    , reducedMotion : Bool
    , visited : List String
    , pages : List Page
    }


type alias Page =
    { id : String
    , title : String
    , links : List String
    , tags : List String
    }


{-| Everything but the page list has a default, because these come from a
config file a reader of this repository is meant to be able to edit.
-}
flagsDecoder : Decoder Flags
flagsDecoder =
    let
        number name fallback =
            Decode.oneOf [ Decode.field name Decode.float, Decode.succeed fallback ]

        yesNo name fallback =
            Decode.oneOf [ Decode.field name Decode.bool, Decode.succeed fallback ]
    in
    Decode.map8
        (\slug base box depth showTags removeTags visited pages ->
            { slug = slug
            , base = base
            , box = box
            , depth = depth
            , showTags = showTags
            , removeTags = removeTags
            , visited = visited
            , pages = pages
            , repelForce = 0.5
            , centerForce = 0.3
            , linkDistance = 30
            , focusOnHover = False
            , enableRadial = False
            , reducedMotion = False
            }
        )
        (Decode.field "slug" Decode.string)
        (Decode.oneOf [ Decode.field "base" Decode.string, Decode.succeed "" ])
        boxDecoder
        (Decode.oneOf [ Decode.field "depth" Decode.int, Decode.succeed 1 ])
        (yesNo "showTags" True)
        (Decode.oneOf [ Decode.field "removeTags" (Decode.list Decode.string), Decode.succeed [] ])
        (Decode.oneOf [ Decode.field "visited" (Decode.list Decode.string), Decode.succeed [] ])
        (Decode.field "pages" (Decode.list pageDecoder))
        |> Decode.andThen
            (\partial ->
                Decode.map6
                    (\repel center distance focus onRing still ->
                        { partial
                            | repelForce = repel
                            , centerForce = center
                            , linkDistance = distance
                            , focusOnHover = focus
                            , enableRadial = onRing
                            , reducedMotion = still
                        }
                    )
                    (number "repelForce" 0.5)
                    (number "centerForce" 0.3)
                    (number "linkDistance" 30)
                    (yesNo "focusOnHover" False)
                    (yesNo "enableRadial" False)
                    (yesNo "reducedMotion" False)
            )


boxDecoder : Decoder Box
boxDecoder =
    Decode.map4 Box
        (Decode.oneOf [ Decode.field "width" Decode.float, Decode.succeed 250 ])
        (Decode.oneOf [ Decode.field "height" Decode.float, Decode.succeed 250 ])
        (Decode.oneOf [ Decode.field "left" Decode.float, Decode.succeed 0 ])
        (Decode.oneOf [ Decode.field "top" Decode.float, Decode.succeed 0 ])


pageDecoder : Decoder Page
pageDecoder =
    Decode.map4 Page
        (Decode.field "id" Decode.string)
        (Decode.oneOf [ Decode.field "title" Decode.string, Decode.succeed "" ])
        (Decode.oneOf [ Decode.field "links" (Decode.list Decode.string), Decode.succeed [] ])
        (Decode.oneOf [ Decode.field "tags" (Decode.list Decode.string), Decode.succeed [] ])



-- MODEL


type Kind
    = PageNode
    | TagNode


type alias Node =
    { id : String
    , label : String
    , kind : Kind
    , degree : Int
    , x : Float
    , y : Float
    , vx : Float
    , vy : Float

    -- Held by the mouse: the forces still push against it and it does not
    -- move, which is what makes dragging one node rearrange the rest.
    , pinned : Bool
    }


{-| A link by node index, carrying the two numbers d3 works out for it once:
how hard it pulls, and how that pull is shared between its ends. A page with
one link is moved much further by it than a hub with twenty.
-}
type alias Link =
    { source : Int
    , target : Int
    , strength : Float
    , bias : Float
    }


{-| The flags are kept whole rather than unpacked: the simulation reads its
settings out of them every frame, and a resize writes a new size back into
them.
-}
type alias Model =
    { flags : Flags
    , nodes : Array Node
    , links : List Link
    , neighbours : Dict String (Set String)
    , alpha : Float
    , hovered : Maybe String
    , visited : Set String
    , camera : Camera
    , drag : Drag

    -- A click arrives after the mouse is let go, and a node is a link. This is
    -- how the one that ends a drag is told from the one that follows it.
    , dragged : Bool
    }


type Msg
    = Tick
    | Hover (Maybe String)
    | Follow String
    | Resized Box
    | Halted
      -- The mouse, in window coordinates: where it went down, where it has got
      -- to, and where it was when the wheel turned.
    | Grabbed String Point
    | Panned Point
    | Moved Point
    | Released
    | Wheeled Float Point



-- THE GRAPH ITSELF


{-| Every link on the site, page to page and page to tag, before the
neighbourhood is cut out of it. A link to a page that is not there is dropped,
which is what stops a typo in a wikilink from drawing a node.
-}
allEdges : Flags -> List ( String, String )
allEdges flags =
    let
        known =
            flags.pages |> List.map .id |> Set.fromList

        removed =
            Set.fromList flags.removeTags

        outgoing page =
            page.links
                |> List.filter (\to -> Set.member to known)
                |> List.map (\to -> ( page.id, to ))

        tagged page =
            if flags.showTags then
                page.tags
                    |> List.filter (\tag -> not (Set.member tag removed))
                    |> List.map (\tag -> ( page.id, "tags/" ++ tag ))

            else
                []
    in
    flags.pages |> List.concatMap (\page -> outgoing page ++ tagged page)


{-| The ids within `depth` steps of the page being read, in either direction
along a link. A negative depth is the whole site, which is what the global
graph asks for.
-}
neighbourhood : Int -> String -> List ( String, String ) -> Set String
neighbourhood depth start edges =
    let
        adjacency =
            List.foldl
                (\( from, to ) acc -> acc |> insertNeighbour from to |> insertNeighbour to from)
                Dict.empty
                edges

        walk remaining frontier seen =
            if remaining <= 0 || Set.isEmpty frontier then
                seen

            else
                let
                    next =
                        frontier
                            |> Set.toList
                            |> List.concatMap
                                (\id ->
                                    Dict.get id adjacency
                                        |> Maybe.withDefault Set.empty
                                        |> Set.toList
                                )
                            |> Set.fromList
                            |> (\found -> Set.diff found seen)
                in
                walk (remaining - 1) next (Set.union seen next)
    in
    if depth < 0 then
        edges
            |> List.concatMap (\( from, to ) -> [ from, to ])
            |> Set.fromList
            |> Set.insert start

    else
        walk depth (Set.singleton start) (Set.singleton start)


insertNeighbour : String -> String -> Dict String (Set String) -> Dict String (Set String)
insertNeighbour from to acc =
    Dict.update from
        (\existing -> Just (Set.insert to (Maybe.withDefault Set.empty existing)))
        acc


{-| d3 scatters its starting positions with `Math.random`, and falls back to a
phyllotaxis spiral -- the sunflower-seed arrangement -- for any node whose
position is not a number. That fallback is used here for all of them: it
spreads nodes evenly with no two on top of each other, and it is the same
spiral every time the page opens.
-}
phyllotaxis : Int -> ( Float, Float )
phyllotaxis index =
    let
        i =
            toFloat index

        distance =
            10 * sqrt (0.5 + i)

        angle =
            i * pi * (3 - sqrt 5)
    in
    ( distance * cos angle, distance * sin angle )


build : Flags -> Model
build flags =
    let
        edges =
            allEdges flags

        inside =
            neighbourhood flags.depth flags.slug edges

        kept =
            edges |> List.filter (\( from, to ) -> Set.member from inside && Set.member to inside)

        titles =
            flags.pages |> List.map (\page -> ( page.id, page.title )) |> Dict.fromList

        degrees =
            List.foldl (\( from, to ) acc -> acc |> bump from |> bump to) Dict.empty kept

        ids =
            Set.toList inside

        index =
            ids |> List.indexedMap (\i id -> ( id, i )) |> Dict.fromList

        node i id =
            let
                ( x, y ) =
                    phyllotaxis i
            in
            { id = id
            , label = label titles id
            , kind =
                if String.startsWith "tags/" id then
                    TagNode

                else
                    PageNode
            , degree = Dict.get id degrees |> Maybe.withDefault 0
            , x = x
            , y = y
            , vx = 0
            , vy = 0
            , pinned = False
            }

        link ( from, to ) =
            Maybe.map2
                (\source target ->
                    let
                        fromDegree =
                            Dict.get from degrees |> Maybe.withDefault 1 |> toFloat

                        toDegree =
                            Dict.get to degrees |> Maybe.withDefault 1 |> toFloat
                    in
                    { source = source
                    , target = target
                    , strength = 1 / max 1 (min fromDegree toDegree)
                    , bias = fromDegree / max 1 (fromDegree + toDegree)
                    }
                )
                (Dict.get from index)
                (Dict.get to index)
    in
    { flags = flags
    , nodes = ids |> List.indexedMap node |> Array.fromList
    , links = kept |> List.filterMap link
    , neighbours =
        List.foldl
            (\( from, to ) acc -> acc |> insertNeighbour from to |> insertNeighbour to from)
            Dict.empty
            kept
    , alpha = 1
    , hovered = Nothing
    , visited = Set.fromList flags.visited
    , camera = resting
    , drag = Still
    , dragged = False
    }


bump : String -> Dict String Int -> Dict String Int
bump id acc =
    Dict.update id (\existing -> Just (1 + Maybe.withDefault 0 existing)) acc


label : Dict String String -> String -> String
label titles id =
    if String.startsWith "tags/" id then
        "#" ++ String.dropLeft 5 id

    else
        case Dict.get id titles of
            Just title ->
                if String.isEmpty title then
                    id

                else
                    title

            Nothing ->
                id



-- THE SIMULATION
--
-- d3-force's numbers in d3-force's order: alpha eases towards zero, each force
-- writes into the velocities, and the positions are integrated last.


alphaDecay : Float
alphaDecay =
    0.0228


alphaMin : Float
alphaMin =
    0.001


velocityDecay : Float
velocityDecay =
    0.6


radius : Node -> Float
radius node =
    2 + sqrt (toFloat node.degree)


{-| The work the first layout is allowed, counted in node pairs.

A force simulation that starts from its seed positions spends its first second
looking like an explosion, which in a sidebar is movement for its own sake. d3
says as much about layouts nobody is going to watch settle: run the ticks
yourself and draw the result. So some of them are run before anything is drawn,
and the animation eases the rest of the way in -- which also means the picture
does not depend on animation frames arriving at all. A graph in a background
tab is drawn laid out rather than as a spiral of untouched seed positions.

A step compares every node with every other, so a graph twice the size costs
four times as much per step. Fixing the number of steps rather than the work is
what made opening the whole-site graph -- 94 nodes against the sidebar's three
-- block the page for 376 ms; against a budget it is nearer forty, and the
sidebar still settles completely before it is drawn.
-}
layoutBudget : Int
layoutBudget =
    110000


{-| A bound on how long the settling can run, whatever the budget allows, so
that a strange graph cannot spin here forever.
-}
restingSteps : Int
restingSteps =
    600


{-| The layout as it is first drawn: settled as far as the budget goes, or
settled and left still for a reader who has asked for less movement.
-}
lay : Model -> Model
lay model =
    let
        pairs =
            max 1 (Array.length model.nodes ^ 2)

        steps allowance =
            clamp 1 restingSteps (allowance // pairs)
    in
    if model.flags.reducedMotion then
        let
            rested =
                settle (steps (4 * layoutBudget)) model
        in
        { rested | alpha = 0 }

    else
        settle (steps layoutBudget) model


{-| Steps until the count runs out, or until the layout has stopped moving. -}
settle : Int -> Model -> Model
settle count model =
    if count <= 0 || model.alpha <= alphaMin then
        model

    else
        settle (count - 1) (step model)


step : Model -> Model
step model =
    let
        alpha =
            model.alpha - model.alpha * alphaDecay

        nodes =
            model.nodes
                |> charge (-100 * model.flags.repelForce) alpha
                |> pull alpha model.flags.linkDistance model.links
                |> ring model alpha
                |> centre model.flags.centerForce
                |> collide
                |> Array.map integrate
    in
    { model | nodes = nodes, alpha = alpha }


{-| Every node pushes every other away. Fifty nodes is 2,500 pairs a frame,
which is nothing; d3 builds a quadtree because it is written for graphs a
hundred times the size of this one.
-}
charge : Float -> Float -> Array Node -> Array Node
charge strength alpha nodes =
    let
        every =
            Array.toIndexedList nodes

        push index node =
            List.foldl
                (\( other, that ) acc ->
                    if other == index then
                        acc

                    else
                        let
                            dx =
                                that.x - acc.x

                            dy =
                                that.y - acc.y

                            squared =
                                max 1.0e-6 (dx * dx + dy * dy)

                            weight =
                                strength * alpha / squared
                        in
                        { acc | vx = acc.vx + dx * weight, vy = acc.vy + dy * weight }
                )
                node
                every
    in
    Array.indexedMap push nodes


{-| Each link pulls its ends towards `distance` apart, sharing the correction
between them by the bias: the end with fewer links of its own gives way.
-}
pull : Float -> Float -> List Link -> Array Node -> Array Node
pull alpha distance links nodes =
    List.foldl (pullOne alpha distance) nodes links


pullOne : Float -> Float -> Link -> Array Node -> Array Node
pullOne alpha distance link nodes =
    case ( Array.get link.source nodes, Array.get link.target nodes ) of
        ( Just source, Just target ) ->
            let
                dx =
                    target.x + target.vx - source.x - source.vx

                dy =
                    target.y + target.vy - source.y - source.vy

                length =
                    max 1.0e-6 (sqrt (dx * dx + dy * dy))

                weight =
                    (length - distance) / length * alpha * link.strength

                mx =
                    dx * weight

                my =
                    dy * weight
            in
            nodes
                |> Array.set link.target
                    { target | vx = target.vx - mx * link.bias, vy = target.vy - my * link.bias }
                |> Array.set link.source
                    { source | vx = source.vx + mx * (1 - link.bias), vy = source.vy + my * (1 - link.bias) }

        _ ->
            nodes


{-| The global graph pulls everything onto a circle, which is what keeps a
whole-site graph from collapsing into a knot in the middle.
-}
ring : Model -> Float -> Array Node -> Array Node
ring model alpha nodes =
    if not model.flags.enableRadial then
        nodes

    else
        let
            wanted =
                min model.flags.box.width model.flags.box.height / 2 * 0.8

            apply node =
                let
                    dx =
                        if node.x == 0 then
                            1.0e-6

                        else
                            node.x

                    dy =
                        if node.y == 0 then
                            1.0e-6

                        else
                            node.y

                    distance =
                        max 1.0e-6 (sqrt (dx * dx + dy * dy))

                    weight =
                        (wanted - distance) * 0.2 * alpha / distance
                in
                { node | vx = node.vx + dx * weight, vy = node.vy + dy * weight }
        in
        Array.map apply nodes


{-| Holds the drawing in the middle of its box. This one moves positions rather
than velocities and is not scaled by alpha, which d3 does too -- it is why the
graph stays centred long after it has stopped moving.
-}
centre : Float -> Array Node -> Array Node
centre strength nodes =
    let
        count =
            toFloat (Array.length nodes)
    in
    if count == 0 then
        nodes

    else
        let
            ( sumX, sumY ) =
                Array.foldl (\node ( x, y ) -> ( x + node.x, y + node.y )) ( 0, 0 ) nodes

            shiftX =
                sumX / count * strength

            shiftY =
                sumY / count * strength
        in
        Array.map (\node -> { node | x = node.x - shiftX, y = node.y - shiftY }) nodes


{-| Keeps two circles off each other. One pass where d3 runs three: the circles
here are three to six pixels across, and a pixel of leftover overlap is gone by
the next frame anyway.
-}
collide : Array Node -> Array Node
collide nodes =
    let
        every =
            Array.toIndexedList nodes

        resolve index node =
            List.foldl
                (\( other, that ) acc ->
                    if other == index then
                        acc

                    else
                        let
                            dx =
                                acc.x + acc.vx - that.x - that.vx

                            dy =
                                acc.y + acc.vy - that.y - that.vy

                            wanted =
                                radius acc + radius that

                            distance =
                                max 1.0e-6 (sqrt (dx * dx + dy * dy))
                        in
                        if distance >= wanted then
                            acc

                        else
                            let
                                weight =
                                    (wanted - distance) / distance * 0.5
                            in
                            { acc | vx = acc.vx + dx * weight, vy = acc.vy + dy * weight }
                )
                node
                every
    in
    Array.indexedMap resolve nodes


integrate : Node -> Node
integrate node =
    if node.pinned then
        -- A node under the mouse stays where the mouse put it, and its
        -- velocity is thrown away rather than saved up: d3 does the same, so
        -- that a node let go after a long drag does not shoot off with
        -- everything the forces wanted to do to it in the meantime.
        { node | vx = 0, vy = 0 }

    else
    let
        vx =
            node.vx * velocityDecay

        vy =
            node.vy * velocityDecay
    in
    { node | x = node.x + vx, y = node.y + vy, vx = vx, vy = vy }



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Tick ->
            ( step model, Cmd.none )

        Hover id ->
            ( { model | hovered = id }, Cmd.none )

        Follow id ->
            -- The click that ends a drag is the same click that follows a
            -- link, and only the mouse knows which it was.
            if model.dragged then
                ( { model | dragged = False }, Cmd.none )

            else
                ( model, follow id )

        Resized box ->
            -- A box of a different size is a different drawing, so the layout
            -- is given some of its energy back rather than left where it was.
            ( { model | flags = withBox box model.flags, alpha = max model.alpha 0.3 }
            , Cmd.none
            )

        Halted ->
            -- Nothing more to do, and nothing more to draw: with the alpha at
            -- rest there is no animation frame to ask for, and the app sits
            -- there costing nothing until it is collected.
            ( { model | alpha = 0 }, Cmd.none )

        Grabbed id at ->
            let
                held =
                    where_ model at
            in
            ( { model
                | drag =
                    Holding
                        { id = id
                        , grab = grabbed model id held
                        , moved = False
                        }
                , nodes = pin id model.nodes

                -- The rest of the graph should get out of the way while a node
                -- is being moved, which it only does with some energy left.
                , alpha = max model.alpha 0.3
                , hovered = Just id
              }
            , Cmd.none
            )

        Panned at ->
            ( { model | drag = Panning { from = at, moved = False } }, Cmd.none )

        Moved at ->
            ( moved at model, Cmd.none )

        Released ->
            ( { model
                | drag = Still
                , dragged = wasMoved model.drag
                , nodes = Array.map (\node -> { node | pinned = False }) model.nodes
              }
            , Cmd.none
            )

        Wheeled delta at ->
            ( { model | camera = zoomed model delta at }, Cmd.none )


{-| Where a point in the window is over the layout.

The `viewBox` is the box's own size divided by the zoom, centred on the camera,
and the aspect ratios match by construction -- so there is no letterboxing to
account for and the mapping is this one line each way.
-}
where_ : Model -> Point -> Point
where_ model at =
    { x = model.camera.x + (at.x - model.flags.box.left - model.flags.box.width / 2) / model.camera.zoom
    , y = model.camera.y + (at.y - model.flags.box.top - model.flags.box.height / 2) / model.camera.zoom
    }


{-| How far the node is from the pointer when it is picked up, so that it does
not jump to sit under the cursor.
-}
grabbed : Model -> String -> Point -> Point
grabbed model id at =
    case model.nodes |> Array.filter (\node -> node.id == id) |> Array.get 0 of
        Just node ->
            { x = at.x - node.x, y = at.y - node.y }

        Nothing ->
            { x = 0, y = 0 }


pin : String -> Array Node -> Array Node
pin id nodes =
    Array.map (\node -> { node | pinned = node.id == id }) nodes


wasMoved : Drag -> Bool
wasMoved drag =
    case drag of
        Still ->
            False

        Holding held ->
            held.moved

        Panning pan ->
            pan.moved


{-| A pixel or two of travel is a click with a shaky hand; more than that is a
drag, and is not a link being followed.
-}
slop : Float
slop =
    3


moved : Point -> Model -> Model
moved at model =
    case model.drag of
        Still ->
            model

        Holding held ->
            let
                to =
                    where_ model at
            in
            { model
                | nodes =
                    Array.map
                        (\node ->
                            if node.id == held.id then
                                { node | x = to.x - held.grab.x, y = to.y - held.grab.y }

                            else
                                node
                        )
                        model.nodes
                , drag = Holding { held | moved = True }
                , alpha = max model.alpha 0.3
            }

        Panning pan ->
            let
                dx =
                    at.x - pan.from.x

                dy =
                    at.y - pan.from.y
            in
            { model
                | camera =
                    { zoom = model.camera.zoom
                    , x = model.camera.x - dx / model.camera.zoom
                    , y = model.camera.y - dy / model.camera.zoom
                    }
                , drag =
                    Panning
                        { from = at
                        , moved = pan.moved || abs dx + abs dy > slop
                        }
            }


{-| Zoom about the pointer: whatever was under it stays under it.

d3 scales by two to the power of the wheel's delta over 500, and so does this,
so a notch of a wheel moves the picture by as much as it used to.
-}
zoomed : Model -> Float -> Point -> Camera
zoomed model delta at =
    let
        before =
            where_ model at

        camera =
            { zoom = clamp furthest closest (model.camera.zoom * (2 ^ (-delta / 500)))
            , x = model.camera.x
            , y = model.camera.y
            }

        after =
            where_ { model | camera = camera } at
    in
    { camera
        | x = camera.x + before.x - after.x
        , y = camera.y + before.y - after.y
    }


withBox : Box -> Flags -> Flags
withBox box flags =
    { flags | box = box }


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ if model.alpha > alphaMin then
            Browser.Events.onAnimationFrame (\_ -> Tick)

          else
            Sub.none
        , resized Resized
        , halt (\() -> Halted)

        -- On the document rather than the node, because a hand that drags
        -- quickly leaves the box behind, and letting go out there still has to
        -- count as letting go.
        , case model.drag of
            Still ->
                Sub.none

            _ ->
                Sub.batch
                    [ Browser.Events.onMouseMove (Decode.map Moved point)
                    , Browser.Events.onMouseUp (Decode.succeed Released)
                    ]
        ]


{-| Where a mouse event happened, in the window. -}
point : Decoder Point
point =
    Decode.map2 Point
        (Decode.field "clientX" Decode.float)
        (Decode.field "clientY" Decode.float)



-- VIEW


href : Model -> String -> String
href model id =
    model.flags.base ++ "/" ++ id


{-| What the camera is looking at, as the four numbers of a `viewBox`.

Zooming and panning are this and nothing else: no transform on the nodes, no
second coordinate system to keep in step, and a stylesheet that goes on
measuring strokes and font sizes in the units it was written in.
-}
viewBox : Model -> String
viewBox model =
    let
        width =
            model.flags.box.width / model.camera.zoom

        height =
            model.flags.box.height / model.camera.zoom
    in
    [ model.camera.x - width / 2
    , model.camera.y - height / 2
    , width
    , height
    ]
        |> List.map String.fromFloat
        |> String.join " "


view : Model -> Html Msg
view model =
    Svg.svg
        [ Attr.class "elm-graph-svg"
        , Attr.viewBox (viewBox model)
        , Attr.preserveAspectRatio "xMidYMid meet"

        -- The page does not scroll while the graph is being zoomed, which is
        -- what a reader reaching for the wheel over a graph means by it.
        , Html.Events.custom "wheel"
            (Decode.map2
                (\delta at ->
                    { message = Wheeled delta at
                    , stopPropagation = False
                    , preventDefault = True
                    }
                )
                wheelDelta
                point
            )
        ]
        [ -- Something to catch a drag that starts on the background, filling
          -- whatever the camera is looking at. `pointer-events` in the
          -- stylesheet keeps it from swallowing anything else.
          Svg.rect
            [ Attr.class "elm-graph-field"
            , Attr.x (String.fromFloat (model.camera.x - model.flags.box.width / model.camera.zoom / 2))
            , Attr.y (String.fromFloat (model.camera.y - model.flags.box.height / model.camera.zoom / 2))
            , Attr.width (String.fromFloat (model.flags.box.width / model.camera.zoom))
            , Attr.height (String.fromFloat (model.flags.box.height / model.camera.zoom))
            , Html.Events.custom "mousedown"
                (Decode.map
                    (\at ->
                        { message = Panned at, stopPropagation = False, preventDefault = True }
                    )
                    point
                )
            ]
            []
        , Svg.g [ Attr.class "elm-graph-links" ] (List.map (viewLink model) model.links)
        , Svg.g [ Attr.class "elm-graph-nodes" ]
            (model.nodes |> Array.toList |> List.map (viewNode model))
        ]


{-| How far the wheel turned, in pixels.

A wheel reports its delta in pixels, lines or pages depending on the device and
the browser, and the numbers are not comparable: a line is worth about 16
pixels and a page about a boxful. Without this a mouse wheel in Firefox, which
reports lines, would zoom a fortieth as far as the same wheel in Chrome.
-}
wheelDelta : Decoder Float
wheelDelta =
    Decode.map2
        (\delta mode ->
            case mode of
                1 ->
                    delta * 16

                2 ->
                    delta * 400

                _ ->
                    delta
        )
        (Decode.field "deltaY" Decode.float)
        (Decode.oneOf [ Decode.field "deltaMode" Decode.int, Decode.succeed 0 ])


{-| How much of a label is showing before anyone hovers anything.

The old graph faded labels in as the reader zoomed, and the arithmetic is its:
nothing at all until the picture is bigger than life size, then up towards
opaque as it grows. Reading fifty labels at once is not reading; reading the
half dozen you have zoomed into is.
-}
labelOpacity : Camera -> Float
labelOpacity camera =
    clamp 0 1 ((camera.zoom - 1) / 3.75)


{-| Which ids the pointer is on or next to. Hovering nothing lights everything,
which is the resting state.
-}
lit : Model -> Maybe (Set String)
lit model =
    model.hovered
        |> Maybe.map
            (\id ->
                Dict.get id model.neighbours
                    |> Maybe.withDefault Set.empty
                    |> Set.insert id
            )


viewLink : Model -> Link -> Svg Msg
viewLink model link =
    case ( Array.get link.source model.nodes, Array.get link.target model.nodes ) of
        ( Just source, Just target ) ->
            let
                -- Nothing hovered is the resting state, and gets no class of
                -- its own: the stylesheet's plain `.elm-graph-link` is what a
                -- graph nobody is pointing at looks like.
                state =
                    case model.hovered of
                        Nothing ->
                            ""

                        Just id ->
                            if source.id == id || target.id == id then
                                " is-lit"

                            else
                                " is-dim"
            in
            Svg.line
                [ Attr.class ("elm-graph-link" ++ state)
                , Attr.x1 (String.fromFloat source.x)
                , Attr.y1 (String.fromFloat source.y)
                , Attr.x2 (String.fromFloat target.x)
                , Attr.y2 (String.fromFloat target.y)
                ]
                []

        _ ->
            Svg.text ""


viewNode : Model -> Node -> Svg Msg
viewNode model node =
    let
        hovered =
            model.hovered == Just node.id

        active =
            case lit model of
                Nothing ->
                    True

                Just ids ->
                    Set.member node.id ids

        classes =
            [ Just "elm-graph-node"
            , if node.id == model.flags.slug then
                Just "is-current"

              else if node.kind == TagNode then
                Just "is-tag"

              else if Set.member node.id model.visited then
                Just "is-visited"

              else
                Nothing
            , if hovered then
                Just "is-hovered"

              else
                Nothing
            , if active || not model.flags.focusOnHover then
                Nothing

              else
                Just "is-dim"
            ]
                |> List.filterMap identity
                |> String.join " "
    in
    Svg.a
        [ Attr.class classes

        -- `href` rather than `xlink:href`: the old form is deprecated, and it
        -- is also invisible to the accessibility tree, so a graph written that
        -- way is a page of circles nothing but a mouse can reach.
        , Html.Attributes.attribute "href" (href model node.id)

        -- Quartz's router reads `href` off whatever anchor a click came from,
        -- and on an SVG anchor that is an SVGAnimatedString rather than a
        -- string. This asks it to leave the click alone; the port below does
        -- the navigating, through that same router, with a URL it can read.
        , Html.Attributes.attribute "data-router-ignore" ""
        , Svg.Events.onMouseOver (Hover (Just node.id))
        , Svg.Events.onMouseOut (Hover Nothing)

        -- Picking a node up. The default is prevented because the browser
        -- would otherwise start dragging the link itself, ghost image and all,
        -- and it is stopped from reaching the background, which would pan.
        , Html.Events.custom "mousedown"
            (Decode.map
                (\at ->
                    { message = Grabbed node.id at
                    , stopPropagation = True
                    , preventDefault = True
                    }
                )
                point
            )
        , Html.Events.custom "click"
            (Decode.succeed
                { message = Follow node.id
                , stopPropagation = True
                , preventDefault = True
                }
            )
        ]
        [ Svg.circle
            [ Attr.cx (String.fromFloat node.x)
            , Attr.cy (String.fromFloat node.y)
            , Attr.r (String.fromFloat (radius node))
            ]
            []
        , Svg.text_
            [ Attr.class "elm-graph-label"
            , Attr.x (String.fromFloat node.x)
            , Attr.y (String.fromFloat (node.y - radius node - 3))
            , Attr.textAnchor "middle"

            -- A presentation attribute, which the stylesheet's `opacity` beats
            -- -- that is what lets hovering a node show its label whatever the
            -- zoom is doing.
            , Attr.opacity (String.fromFloat (labelOpacity model.camera))
            ]
            [ Svg.text node.label ]
        ]



-- MAIN


main : Program Decode.Value Model Msg
main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }


init : Decode.Value -> ( Model, Cmd Msg )
init raw =
    case Decode.decodeValue flagsDecoder raw of
        Ok flags ->
            ( lay (build flags), Cmd.none )

        Err error ->
            -- An empty graph rather than a broken page: the sidebar simply has
            -- nothing in it, which is what it looked like before it had a
            -- graph, and the console gets told why.
            ( build flags0, failed (Decode.errorToString error) )


{-| Flags for a graph with nothing in it.
-}
flags0 : Flags
flags0 =
    { slug = ""
    , base = ""
    , box = { width = 250, height = 250, left = 0, top = 0 }
    , depth = 1
    , showTags = False
    , removeTags = []
    , repelForce = 0.5
    , centerForce = 0.3
    , linkDistance = 30
    , focusOnHover = False
    , enableRadial = False
    , reducedMotion = False
    , visited = []
    , pages = []
    }
