param (
    # the commit/tag/branch/whatever to start the changelog from
    [Parameter(Mandatory=$true)]
    [string]$logFrom,

    # the commit/tag/branch/whatever to end the changelog to
    [Parameter(Mandatory=$true)]
    [string]$logTo
)

# store a list of matches
# this is not a list of strings, so it behaves strangely
$changelog = git log --pretty=format:%B "^$logFrom" $logTo | Select-String "^[+-] "

# convert the matches to a list of strings
$changelog = @($changelog | % { $_.ToString() })

# list of changes
$changelog
