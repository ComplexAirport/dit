1. Implement resetting to commits
2. After doing all of this, it will be possible to implement switching to other branches (with soft, hard modes, etc.), deleting, etc.
3. Then we can implement diffs
4. in the end, we can do optimizations. Compress blobs using zstd, make files use bincode or messagepack instead of json, optimize algorithms/memory usage, etc.