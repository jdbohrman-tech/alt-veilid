# Contributing to Veilid
Before you get started, please review our [Code of Conduct](./CODE_OF_CONDUCT.md). We're here to make things better and we cannot do that by treating each other without respect.


## Code Contributions
To begin crafting code to contribution to the Veilid project, first set up a [development environment]. After cloning the project, check out a new local branch and name it in a way that describes the work being done. This is refered to as a topic branch.

Some contributions might introduce changes that are incompatible with other existing nodes. In this case it is recommended to also set a [development network]. *NEED TO WRITE*

Once you have added your new function or addressed a bug, test it locally to ensure it's working as expected. If needed, test your work in a development network with more than one node based on your code. Once you're satisfied your code works as intended and does not introduce negative results or new bugs, follow the merge requests section below to submit your work for maintainer review.

We try to consider all merge requests fairly and with attention deserving to those willing to put in time and effort, but if you do not follow these rules, your contribution
will be closed. We strive to ensure that the code joining the main branch is written to a high standard.


### Code Contribution Do's & Don'ts:

Keeping the following in mind gives your contribution the best chance of landing!

#### <u>Merge Requests</u>

* **Do** create a [topic branch] to work on instead of working directly on `main`. This helps to:
	* Protect the process.
	* Ensures users are aware of commits on the branch being considered for merge.
	* Allows for a location for more commits to be offered without mingling with other contributor changes.
	* Allows contributors to make progress while a PR is still being reviewed.
* **Do** follow the [50/72 rule] for Git commit messages.
* **Do** target your merge request to the **main branch**.
* **Do** specify a descriptive title to make searching for your merge request easier.
* **Do** list [verification steps] so your code is testable.
* **Do** [reference associated issues] in your merge request description.
* **Don't** leave your merge request description blank.
* **Don't** abandon your merge request. Being responsive helps us land your code faster.
* **Don't** submit unfinished code.



## Contributions Without Writing Code
There are numerous ways you can contribute to the growth and success of the Veilid project without writing code:

 - [submit]() bugs as well as feature/enhancement requests. Letting us know you found a bug, have an idea for a new feature, or see a way we can enhance existing features is just as important and useful as writing the code related to those things. Send us detailed information about your issue or idea:
 	- Features/Enhancements: Describe your idea. If you're able to, sketch out a diagram or mock-up.
 	- Bugs: Please be sure to include the expected behavior, the observed behavior, and steps to reproduce the problem. Please be descriptive about the environment you've installed your node or application into. More information can be found [below](#bug-reports).
 - [Help other users with open issues]. Sometimes all an issue needs is a little conversation to clear up a process or misunderstanding. Please keep the [Code of Conduct](./CODE_OF_CONDUCT.md) in mind.
 - Help other contributors test recently submitted merge requests. By pulling down a merge request and testing it, you can help validate new code contributions for stability and quality.
 - Report a security or privacy vulnerability. Please let us know if you find ways in which Veilid could handle security and/or privacy in a different or better way. Surely let us know if you find broken or otherwise flawed security and/or privacy functions. You can report these directly to security@veilid.org or by using the form found [on our site]() *NEED TO BUILD*
 - Add or edit documentation. Documentation is a living and evolving library of knowledge. As such, care, feeding, and even pruning is needed from time to time. If you're a non-native english speaker, you can help by replacing any ambiguous idioms, metaphors, or unclear language that might make our documentation hard to understand.


#### <u>Bug Fixes</u>
* **Do** include reproduction steps in the form of verification steps.
* **Do** link to any corresponding [Issues] in the format of `See #1234` in your commit description.

## Bug Reports

Please report vulnerabilities in Veilid directly to security@veilid.org or by using the form found [on our site]() *NEED TO BUILD*. More information about our disclosure policy and Veilid Foundation's approach to coordinated disclosure can be found [on our site](). *NEED TO WRITE*

When reporting Veilid issues:
* **Do** write a detailed description of your bug and use a descriptive title.
* **Do** include reproduction steps, stack traces, and anything that might help us fix your bug.
* **Don't** file duplicate reports. Search open issues for similar bugs before filing a new report.
* **Don't** attempt to report issues on a closed PR. New issues should be openned against the `main` branch.

If you're looking for more guidance, talk to other Veilid contributors on the [Veilid Discord].

**Thank you** for taking the few moments to read this far! Together we will build something truely remarkable.



This contributor guide is inspired by the contribution guidelines of the [Metasploit Framework](https://github.com/rapid7/metasploit-framework/blob/master/CONTRIBUTING.md) project found on GitHub.

[Code of Conduct]:https://docs.metasploit.com/docs/code-of-conduct.html
[Submit bugs and feature requests]:http://r-7.co/MSF-BUGv1
[Help fellow users with open issues]:https://github.com/rapid7/metasploit-framework/issues
[help fellow committers test recently submitted merge requests]:https://github.com/rapid7/metasploit-framework/pulls
[Report a security vulnerability in Metasploit itself]:https://www.rapid7.com/disclosure.jsp
[development environment]:http://r-7.co/MSF-DEV
[proof-of-concept exploits]:https://www.exploit-db.com/search?verified=true&hasapp=true&nomsf=true
[Ruby style guide]:https://github.com/bbatsov/ruby-style-guide
[Rubocop]:https://rubygems.org/search?query=rubocop
[50/72 rule]:http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[topic branch]:http://git-scm.com/book/en/Git-Branching-Branching-Workflows#Topic-Branches
[draft PR]:https://help.github.com/en/articles/about-pull-requests#draft-pull-requests
[console output]:https://docs.github.com/en/free-pro-team@latest/github/writing-on-github/creating-and-highlighting-code-blocks#fenced-code-blocks
[verification steps]:https://docs.github.com/en/free-pro-team@latest/github/writing-on-github/basic-writing-and-formatting-syntax#task-lists
[reference associated issues]:https://github.com/blog/1506-closing-issues-via-pull-requests
[PR#9966]:https://github.com/rapid7/metasploit-framework/pull/9966
[pre-commit hook]:https://github.com/rapid7/metasploit-framework/blob/master/tools/dev/pre-commit-hook.rb
[API]:https://rapid7.github.io/metasploit-framework/api
[module documentation]:https://docs.metasploit.com/docs/using-metasploit/basics/module-documentation.html
[scripts]:https://github.com/rapid7/metasploit-framework/tree/master/scripts
[RSpec]:http://rspec.info
[Better Specs]:http://www.betterspecs.org/
[YARD]:http://yardoc.org
[Issues]:https://github.com/rapid7/metasploit-framework/issues
[Metasploit Slack]:https://www.metasploit.com/slack
[#metasploit on Freenode IRC]:http://webchat.freenode.net/?channels=%23metasploit&uio=d4
