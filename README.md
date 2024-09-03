# imglang

The **im**age **g**eneration **lang**uage

## Project goals

As a start, I will just layout my rough goals for this project, before writing any actual code. Between my full-time job and caring for my son, there is almost no time left for any kind of side project I'm willing to allocate. So for now, writing down ideas comes first, mending them into the shape of written code comes second, or *never*.

I'm yarning for some kind of tool, that allows me to programatically generate images (or animations) with focus on *ease-of-prototyping*. The imagery I'm creating is mostly for UI, or explanatory purposes, so visual fidelity is not of upmost importance, but *legibility* surely is. Also typically an image is rather a *family* of images depending on some set of external *states*. Think f.e. *enabled/disabled*, or *sub-higlighting* of image parts. For this it's important that the image can be *parameterized* and *(re)*generated via some given parameters.

An approach towards this goal, that I want to test in this repo is as follows:

- Implement some simple geometric scripting language
  - This language should have a way to define *inputs* and *outputs*
  - *inputs* will be primitive data types, as well as colors. It is important that a *range* of valid values for them can be defined
  - *outputs* for now will be a collection of geometric shapes to draw, as well as *stroke* information for them. Probably outputs can also be more generally allowed to take any shape in the future.
- Implement a *parser* + *evaluator* for said scripting language
- Build a cli that allows passing a script, as well as it's inputs and returns the generated image (or saves it as a file)
- Wrap the evaluator into an *self-generating* UI which allows for quick prototyping
  - There should be a simple text editor (maybe monaco)?
  - The *inputs* of the script should be parsed and transformed into simple ui elements f.e. *sliders*, *toggles*, *color-pickers*, etc.
  - The generated image should be displayed inside of the UI

  ### Further steps

  If i feel the scripting language will be too cumbersome to use, or too slow to iterate with, maybe I'll implement a *node-based* visual scripting interface.

  ## Technology Stack

  ### Scripting language

  The language will be written in *Rust*, as my top goal is parsing and evaluating speed and Rust is the fastest language I know. I'm leaning towards a functional flavor for the language, as geometric programming is math-heavy at it's core, but nothing is decided there yet.

  ### Cli

  As it needs to call the evaluator, either also *Rust* with *Clap*, or I expose a *FFI* interface and use python for the cli. It's main goal is simplicity and ease of use, so whatever works best in that field will be the choice.

  ### UI

  The UI needs to have a very tight feedback loop with the evaluator, so probably must be *Rust*, too. Important factors is the availability of a good text-editor control, as I don't want to bike-shed that, instead of working towards the actual project goals. Exposing the evaluator via *PyO3* and using a python UI toolkit might be a possibility too. Alternatively, anything that compiles to web should have good support for hot-relading the *inputs*, as those will frequently change while prototyping and must be quick to reload

  ## Design Ideas

  I'll loosely write down my thoughts during design of the individual components. This section probably will be edited and changed quite heavily, to stay updated with the latest developments.

  ### imglang - the language

  I've been looking into functional languages more and more recently and my *gut-feeling* is that they map nicely to the problem domain of geometric scripts. Those scripts typically consist of multiple pipelines of data streams that combine or branch off again at multiple points. A simple example from the top of my head could be:

  - Create a circle at the origin $O = (0, 0)$, with radius $r$
  - Sample $n$ points, *uniformly* along the *domain* of the circle
  - Draw a line with thickness $t$ from each sampled point to the origin $O$
  - Draw the original circle with thickness $t * 2$

  This would generate a very simple drawing of a bike wheel *- coincidentially the perfect bike-shedding analogy for this project -* with the ability to change quite a lot of drawing parameters.

  Let's now look at some *pseudo-syntax* of how a script like this might look, to see what *feels* good. In my opinion, programming must be fun, so the feel of the language is **immensely** important. First s functional-ish approach borrowing some ideas from *Gleam, OCaml, F#, etc*

  ```fsharp
  // define the script inputs
  // do note the extern keyword
  let extern r = externInt 10 15 // integer between 10 and 15 inclusive
  let extern n = externInt 3 7
  let extern t = externFloat 0.2 1.0

  // define the canvas
  let canvas = canvas 400 300 // good old 4:3 aspect ratio
  
  // define strokes for wheel and spikes
  let wheelStroke = stroke BLACK t * 2
  let spikeStroke = stroke DARKGRAY t

  // define the wheel curve
  let wheel = circleOriginRadius r
  // define the spiked
  let spikes =
      wheel
      |> samplePointsUniform n
      |> lineTo ORIGIN

  // draw the geometry to the canvas
  canvas
  |> draw wheel wheelStroke // the draw function returns the updated canvas, so we can keep on piping
  |> drawMany spikes spikeStroke 
  |> out
  ```

I want Scripts to Compose nicely so some language Features will be needed:

- [ ] extern Inputs
- [ ] primitive constants
- [ ] dedicated Outputs
- [ ] functions
- [ ] partial application
- [ ] tagged Unions
- [ ] Pattern matching

although simplicity will Always be the Main Driver when deciding on adding Features. additionally, some thought should go early into how we could allow for animations to be generated, too. I feel some elm-like `MVU` Pattern would map nicely on the Code Side,  but mapping the result to an animated vector Format (Like *SVG*) could be potentially awkward.

For a quick and nice Parser Im learning towards Just using winnow for now. A glance over the documentation looks quite nice. For inspiration I can probably look at `gleam`, as the syntax is quite similar for what I'm aiming for (though I don't want to do curly braces, if i can avoid them). I think a big strength of python is that it sometimes allows you to **just write** without thinking about syntax or semantics. That's where I'd like imglang to be. Whitespace has the tendency to get annoying and to produce very hard to debug errors, so I'm not fully sold on it yet... 

The `ast` will be hopefully quite simple to construct, though I'm quite hesitant with some of the listed language features, as they will make for some complex structs in the end. I'll follow the advice of `matklad` and prioritize getting something, or rather *anything* working first and following up by expanding the features later.

### MVP

To really condense it down, the MVP needs to to only do a few things at first:

- [ ] Some scripting language with a tiny stdlib allowing for very basic geometric scripting
  - The language should be reduced to what is seen in the sample script, so only function calling of stdlib functions, as well as defining variables
- [ ] A basic compiler that can generate some image format from the script. Whatever is easiest to implement. In the future I'll want `piet` as a kind of *intermediate representation*, as this unlocks quite a few image formats out of the box, as well as vectors, which for me is the most important format.
- [ ] *optional*: "ide" for the language, where inputs of the script are parsed and automatically rendered as a UI, allowing to re-execute the compiler cli with the values from the UI set and displaying the rendered image. Would be best if it could compile to web, so the experience of using it is easy to set up and requires no big downloads, but I'm open to any UI framework, as the first prototype will be super simple anyways and since it's optional, *speed of implementation* will be the deciding factor.

I'll also go with the most basic script first, that needs to compile

```fsharp
  // radius input of circle to draw
  let extern r = externInt 10 15 // integer between 10 and 15 inclusive

  // define the canvas
  let canvas = canvasWidthHeight 400 300 // good old 4:3 aspect ratio
  
  // define strokes for curve
  let circleStroke = stroke BLACK r / 10.0

  // define the curve to draw
  let circle = circleOriginRadius r

  canvas
  |> draw circle circleStroke
  |> out
```

