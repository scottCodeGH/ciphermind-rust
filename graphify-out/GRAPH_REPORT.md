# Graph Report - ciphermind-rust  (2026-07-07)

## Corpus Check
- 4 files · ~3,148 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 53 nodes · 67 edges · 9 communities (7 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d1787878`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]

## God Nodes (most connected - your core abstractions)
1. `🧩 CipherMind` - 11 edges
2. `Game` - 5 edges
3. `print_colored_symbol()` - 4 edges
4. `🛠️ Technical Details` - 4 edges
5. `🚀 Quick Start` - 3 edges
6. `Difficulty` - 2 edges
7. `test_feedback_all_exact()` - 2 edges
8. `test_feedback_no_matches()` - 2 edges
9. `test_feedback_color_matches()` - 2 edges
10. `test_feedback_mixed()` - 2 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Communities (9 total, 2 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.15
Nodes (12): 🌟 Acknowledgments, 🧩 CipherMind, code:block2 (🎯 Enter your guess: RGYB), 🤝 Contributing, 🎚️ Difficulty Modes, Example Gameplay, ✨ Features, 🎮 Game Overview (+4 more)

### Community 1 - "Community 1"
Cohesion: 0.31
Nodes (7): Feedback, test_easy_feedback_four_slots(), test_feedback_all_exact(), test_feedback_color_matches(), test_feedback_mixed(), test_feedback_no_matches(), test_hard_feedback_five_slots()

### Community 2 - "Community 2"
Cohesion: 0.36
Nodes (4): print_colored_symbol(), print_welcome(), reveal_code(), show_guess_result()

### Community 3 - "Community 3"
Cohesion: 0.40
Nodes (5): Architecture, code:bash (cargo test), Key Functions, Running Tests, 🛠️ Technical Details

### Community 4 - "Community 4"
Cohesion: 0.50
Nodes (4): code:bash (# Clone the repository), Installation & Running, Prerequisites, 🚀 Quick Start

### Community 5 - "Community 5"
Cohesion: 0.33
Nodes (5): test_easy_rejects_colors_outside_palette(), test_hard_requires_five_slots(), test_validate_guess_invalid_color(), test_validate_guess_invalid_length(), test_validate_guess_valid()

## Knowledge Gaps
- **14 isolated node(s):** `Feedback`, `🎚️ Difficulty Modes`, `✨ Features`, `Prerequisites`, `code:bash (# Clone the repository)` (+9 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `🧩 CipherMind` connect `Community 0` to `Community 3`, `Community 4`?**
  _High betweenness centrality (0.143) - this node is a cross-community bridge._
- **Why does `🛠️ Technical Details` connect `Community 3` to `Community 0`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Why does `🚀 Quick Start` connect `Community 4` to `Community 0`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._
- **What connects `Feedback`, `🎚️ Difficulty Modes`, `✨ Features` to the rest of the system?**
  _14 weakly-connected nodes found - possible documentation gaps or missing edges._