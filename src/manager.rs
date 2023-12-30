mod manager {
    // These entries are all delimited by a "="
    // E.g. alias teemo='teemo'
    struct Entry {
        alias: String, // alias
    }
    // Also delimited by "="
    // PATH="/usr/bin:/usr/local/bin:/home/teemo/.cargo/bin"
    struct Variable {
        name: String, // name
        value: String, // entry. 
    }
}