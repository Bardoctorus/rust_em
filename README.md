# Rust YouTube and LibSyn Description Generator

Will be Rust parser type thing not dissimilar to the [EM Parser]("https://github.com/Bardoctorus/EMparser") I made with some updates and with more Rust goodness

I had to nuke it all as it was lots of things, none of which were the parser. All of the code that was in it is now in the unforunately named text_anal project.

# Current Plan

The code I have in place for generating YouTube and Libsyn podcast text is functional but clunky. If you are interested in how it works, see the link in the first sentence of this README as I'll likely keep it around in case I fancy refactoring some Python one day.

The plan for this one is to make a more generic and user friendly description text and link generator, that takes a bunch of Titles/topics, timestamps, and urls, and gives back everything you'd need to post in your YouTube/Podcast description.

I'm also hoping to add sheets integration, to document every topic, link etc, with datestamps to make it easy to see what was talked about when. In my case it's useful for work, but I can't help but think that being able to just search a topic name in excel is better than fighting with YouTube's interface.

Still to decide:

- Input format - sheet?
- Output format - same sheet? Redundant?
- how generic? Could this be sectional and made into a lib?

## Current TODO

-[ ] Implement seperate file for YT Desc Template Text
-[ ] Implement seperate file for Podcast Desc Text
-[ ] Implement input file
-[ ] Implement parsing cases with no timestamp/link
-[ ] Implement saving to dated spreadsheet
-[ ] Look into Sheets integration as possible backup
