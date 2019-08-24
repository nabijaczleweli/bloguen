bloguen(1) -- Generate an ePub book from a simple plaintext descriptor
======================================================================

## SYNOPSIS

`bloguen` IN_DIR OUT_DIR

## DESCRIPTION

Generate an ePub book from a simple plaintext descriptor.

Exit values and possible errors:

    1 - I/O error
    2 - parsing error
    3 - file not found
    4 - file in wrong state
    5 - file parsing failed
    6 - required element missing

## OPTIONS

  IN_DIR

    File to parse, must exist, must comply with the DESCRIPTOR FORMAT.

  OUT_DIR

    File to write the book to, parent directory needn't exist.

## DESCRIPTOR FORMAT

Blogue descriptors are TOML files named `blogue.toml`,
where all keys except for `name` are optional:

    # The blogue's display name.
    name = 'Блогг'

    # The blogue's main author(s).
    #
    # Overriden by post metadata, if present.
    #
    # If not present, defaults to the current system user's name,
    # which, if not detected, errors out.
    author = 'nabijaczleweli'

    # Data to put before post HTML, templated.
    #
    # Default: `"$ROOT/header.html"`, then `"$ROOT/header.htm"`.
    header_file = 'header.html'

    # Data to put after post HTML, templated.
    #
    # Default: `"$ROOT/footer.html"`, then `"$ROOT/footer.htm"`.
    footer_file = 'footer.html'

    # Subfolder to move assets to, relative to the output root, if present.
    #
    # The value is stripped of leading slashes.
    # All backslashes are normalised to forward ones.
    # The value is ended off with a slash, if not already specified.
    #
    # No override is applied if not present – assets are copied alongside the posts' HTML.
    asset_dir_override = 'assets/'

    # Default post language.
    #
    # Overriden by post metadata, if present.
    #
    # If not present, defaults to the current system language,
    # which, if not detected, defaults to en-GB.
    language = 'en-GB'

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    styles = ['link://nabijaczleweli.xyz/kaschism/assets/column.css'
              'literal:.indented { text-indent: 1em; }',
              'file:common.css']

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    scripts = ['link:/content/assets/syllable.js',
               'literal:document.getElementById(\"title\").innerText = \"Наган\";',
               'file:MathJax-config.js']

    # Where and which machine datasets to put.
    #
    # Each value here is a prefix appended to the output directory
    # under which to put the machine data.
    #
    # Values can't be empty (to put machine data at post root use "./").
    [machine_data]
    JSON = 'machine/'

    # Where and which feeds to put.
    #
    # Each value here is a file path appended to the output directory
    # into which to put the machine data.
    [feeds]
    RSS = 'feeds/rss.xml'
    Atom = 'atom.xml'

    # Additional static data to substitute in header and footer for all posts.
    #
    # If not present, defaults to empty.
    [data]
    key_1 = 'data_1'
    key_2 = 'data_2'

    # Metadata specifying how to generate the blogue index file.
    #
    # If not present, index not generated.
    #
    # All keys are optional
    [index]
    # Data to put start index HTML with, templated.
    #
    # Default: `"$ROOT/index_header.html"`, then `"$ROOT/index_header.htm"`,
    #     then `"$ROOT/idx_header.html"`, then `"$ROOT/idx_header.htm"`.
    header_file = 'index_header.html'

    # Data to put in index HTML for each post, templated.
    #
    # Default: `"$ROOT/index_center.html"`, then `"$ROOT/index_center.htm"`,
    #     then `"$ROOT/idx_center.html"`, then `"$ROOT/idx_center.htm"`.
    center_file = 'index_center.html'

    # Data to put to end index HTML with, templated.
    #
    # Default: `"$ROOT/index_footer.html"`, then `"$ROOT/index_footer.htm"`,
    #     then `"$ROOT/idx_footer.html"`, then `"$ROOT/idx_footer.htm"`.
    footer_file = 'index_footer.html'

    # The order to put center templates in.
    #
    # If not present, defaults to forward.
    center_order = "forward|backward"

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    styles = ['link://nabijaczleweli.xyz/kaschism/assets/column.css'
              'literal:.indented { text-indent: 1em; }',
              'file:common.css']

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    scripts = ['link:/content/assets/syllable.js',
               'literal:document.getElementById(\"title\").innerText = \"Наган\";',
               'file:MathJax-config.js']

    # Additional static data to substitute in header and footer.
    #
    # If not present, defaults to empty.
    [data]
    index_key_1 = 'index_data_1'
    index_key_2 = 'index_data_2'

## POST DISCOVERY

Posts reside in directories parallel to `blogue.toml`,
whose names match `#+. YYYY-MM-DD [HH-MM[-SS]] name`,
in files named `post.md`.

In addition to `post.md`, the folder may contain
automatically-copied assets,
a `tags` file containing one tag per line,
and a `metadata.toml`, which obeys the [METADATA FORMAT](#METADATA-FORMAT)

## METADATA FORMAT

Additional post metadata is contained in files named `metadata.toml`, where all keys are optional:

    # Post language override.
    #
    # If not present, default post language is used.
    language = "pl"

    # Post author override.
    #
    # If not present, default post author is used.
    author = "Enet4"

    # A set of tags.
    #
    # If not present, defaults to empty.
    #
    # Added to tags in the tags file
    tags = ["maths", "abstract"]

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    styles = ['link://nabijaczleweli.xyz/kaschism/assets/column.css'
              'literal:.indented { text-indent: 1em; }',
              'file:common.css']

    # A set of style descriptors.
    #
    # If not present, defaults to empty.
    scripts = ['link:/content/assets/syllable.js',
               'literal:document.getElementById(\"title\").innerText = \"Наган\";',
               'file:MathJax-config.js']

    # Additional static data to substitute in header and footer.
    #
    # If not present, defaults to empty.
    [data]
    post_key_1 = 'post_data_1'
    post_key_2 = 'post_data_2'

## FORMAT FORMAT

The post header and footer, as well as index header, center, and footer are formatted in a Rust-format-like fashion,
where `{var}` denotes the insertion of variable `var`, and `{{`/`}}` literal `{`/`}`.

    language                – post language in BCP47 format
                            – en-GB
    number                  – default-formatted post number
                            – 14
    title                   – post title
                            – release-front - a generic release front-end
    author                  – post author
                            – nabijaczleweli
    raw_post_name           – post name as it appeared on the filesystem
                            – 004. 2018-03-30 Stir plate
    normalised_post_name    – normalised post name
                            – 004. 2018-03-30 06-00-51 Stir plate
    blog_name               – blog name
                            – Блогг
    bloguen-version         – current version of bloguen
                            – v0.1.0
    tags                    – ↓
                            – <span class="post-tag">maths</span>…
    tags()                  – all post tags with the default class (post-tag)
                            – <span class="post-tag">maths</span>…
    tags(class)             – all post tags with the specified class, headers and footers
                            – <span class="пост-таг">maths</span>…
    styles                  – all post styles with their headers and footers
                            – <style type="text/css">* {color: magenta;}</style>…
    scripts                 – all post scripts with their headers and footers
                            – <script type="text/javascript">alert("hewwo")</script>…
    data-name               – passed-in data under the name key
                            – hewwo
    date(post, format)      – post date formatted with DATE FORMAT
                            – Thu,  6 Sep 2018 18:32:22 +0200
    date(now_utc, format)   – current date in UTC formatted with DATE FORMAT
                            – Thu,  6 Sep 2018 18:32:22 +0200
    date(now_local, format) – current date in local timezone formatted with DATE FORMAT
                            – Thu,  6 Sep 2018 18:32:22 +0200
    machine_data(kind)      – machine data of the specified kind
                            – {"number": 3, "language": "en-GB", …}…
    pass_paragraphs(n, var) – parse var and write up to n HTML paragraphs of its contents
                            – <p>Paragraph 1</p> <p>Paragraph 2</p>…

## DATE FORMAT

Any of: rfc2822, rfc_2822, RFC2822, RFC_2822 – RFC2822

Any of: rfc3339, rfc_3339, RFC3339, RFC_3339 – RFC3339

Anything else: `strftime()` format

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/bloguen/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/bloguen>&gt;
