1. Firstly, change the head file to either point to branch name or a particular commit. Currently, it can only point to a branch.
2. After changing that, the app can now be in a "detached head" state when there is no current branch there is a commit. Currently, the app cannot exist without pointing to a specific branch
3. After doing all of this, it will be possible to implement switching to other branches (with soft, hard modes, etc.), deleting, etc.
4. Then we can implement resetting to commits
5. Then we can implement diffs
6. in the end, we can do optimizations. Compress blobs using zstd, make files use bincode or messagepack instead of json, optimize algorithms/memory usage, etc.