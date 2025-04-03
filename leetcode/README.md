# NeetCode 150 Rust Implementation

[neetcode 150 problems](./neetcode150.txt)

This project contains Rust implementations of the NeetCode 150 problems from https://neetcode.io/practice?tab=neetcode150.

The rust scraper code is put in the `scraper` folder and the scraper should generate a rust project structure like the below:

neetcode150/
├── Cargo.toml
├── .gitignore
├── src/
│   ├── lib.rs           # Defines shared code/modules
│   ├── common/          # Directory for common modules
│   │   ├── mod.rs       # Declares modules within common/
│   │   ├── list_node.rs # Definition for ListNode
│   │   └── tree_node.rs # Definition for TreeNode
│   └── bin/             # Each file is a separate runnable binary
│       ├── p0001_two_sum.rs
│       ├── p0002_add_two_numbers.rs
│       └── p0003_longest_substring.rs
│       └── ...
└── README.md            # Optional: Track progress, notes


page structure:

```javascript
a = document.querySelectorAll('app-pattern-table tbody');
for (let i = 0; i < a.length; i++) {
    b = a[i].querySelectorAll('tr');
    for (let j = 0; j < b.length; j++) {
        problem_title = b[j].querySelector('td:nth-child(3) a:nth-child(1)');
        leetcode_link = b[j].querySelector('td:nth-child(3) a:nth-child(2)');
        console.log(`${problem_title.textContent} ${leetcode_link.getAttribute('href')}`)
    }
}
```

```text
foreach `app-pattern-table tbody` {
    foreach `tr` {
        yield (`td:nth-child(3) a:nth-child(1)`.innerText, `td:nth-child(3) a:nth-child(2)`[href])
    }
}
```
