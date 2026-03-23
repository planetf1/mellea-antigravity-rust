here is an empty directory. In ../mellea-f we have the python implementation of mellea https://github.com/generative-computing/mellea . I am wondering if we
  could create a POC of a rust version of mellea. We'd need to include a sample app, some tests, and implement some basic mellea function - evaluate what that
  could be, but perhaps the ivr loop and generative functinos -- consider what the core value prop is. I expect that means a rust crate is built. Can you
  initialize everything (including git repo) of this project, and a readme positioning the poc. For llm I suggest sticking with ollama if possible (default
  port) [ ideal if option to use lmstudio as it's quicker ]. Maybe we should start be writing a spec of what we need and saving it, then building off it. It may
  also be useful to use 'beads' (the bd command) if available to track tasks/dependencies/bugs ? Any questions?

   yes create that spec. Compare against other rust llm libraries. Consider best practice/elegance/being language natural as super important aspects. the
  principle would be to ensure that rust users of llms can get better results with mellea than without. Look at common libraries out there for llm usage and
  ollama etc.

  we should use existing crates and buid on top as long as they are regardded as decent quality/used/github stars are decent. Not some one-person no-one uses
  library

  the spec should be clear on the reason for the poc and what it's trying to show, and what benefits mellea-rust would have

  as a code example we'd ideally want before/after examples showing without mellea & with mellea? That would help sell the idea

  the spec should be clear on the reason for the poc and what it's trying to show, and what benefits mellea-rust would have
  as a code example we'd ideally want before/after examples showing without mellea & with mellea? That would help sell the idea

  make sure we explain how we adopt rust idions rather than just port python - maybe a comparison table will help explain to python developers (and rust
  developers)

  ensure we use *current* rust versions (and crates). No need to be compat with past. You can check what version I have installed? That should be latest? Document that too

  ensure rust functions/files etc are nicely documented (just as we have docstrings in python). ideally build doics too if there's a standard rust way (if not
  this can be done later)

  I take it we have tests at each loevel too

  check the example run. Make sure we have tests at all levels - ideally including automatic running of the example. May need to validate ollama etc is running

  we can ensure we checkin our 'spec' too -

  obviously we need to ensure proper error reporting/checks from the runtime results from any external call such as to llm

  so in the examples I assume we have a 'before' and 'after' the 'before' example should run, but may fail/return bad results. that is fine. whils tthe mellea
  example should be more constrained and pass any quality etsts. I hope the example/doc/code shows that


