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
  let r = externInt 10 15 // integer between 10 and 15 inclusive
  let n = externInt 3 7
  let t = externFloat 0.2 1.0

  // define the canvas
  let canvas = canvas 400 300 // good old 4:3 aspect ratio
  
  // define strokes for wheel and spikes
  let circleStroke = stroke BLACK t * 2
  let spikeStroke = stroke DARKGRAY t

  // TODO: How to pass in the canvas purely? inside the draw function?
  // but then draw HAS TO return a new instance of canvas back, which
  // makes piping geometry more awkward

  // do the actual logic
  circleOriginRadius r
  |> draw circleStroke // the draw function returns the geometry passed in, so we can keep on piping
  |> samplePointsUniform n
  |> lineTo ORIGIN
  |> draw spikeStroke // draw should be able to take lists of data, too
  ```
