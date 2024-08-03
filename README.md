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
