app "hello"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.1.1/zAoiC9xtQPHywYk350_b7ust04BmWLW00sjb9ZPtSQk.tar.br" }
    imports [
      pf.Process,
      pf.Stdout,
      pf.File,
      pf.Task.{ Task },
      pf.Path,
      pf.Stderr,
    ]
    provides [main] to pf

# maxElf = \cals amount max ->
#    when cals is
#       [] -> max
#       [h, ..] -> maxElf ™

main : Task {} []
main =
    # path = Path.fromStr "sample.txt"
    path = Path.fromStr "input.txt"
    task =
      contents <- File.readUtf8 path |> Task.await
      lines = Str.split contents "\n\n"
      # joined = Str.joinWith lines "--"
      groups = List.map lines \l -> 
         cals = Str.split l "\n"
         total = List.walk cals 0 \acc, s ->
            n = Str.toI32 s |> Result.withDefault 0
            acc + n
         total
      part1 = List.max groups |> Result.withDefault -1 |> Num.toStr
      # Stdout.line "File contents: \(joined)"

      sorted = List.sortDesc groups
      top3 = List.takeFirst sorted 3
      part2 = List.sum top3 |> Num.toStr

      # Stdout.line("File contents: \(m)")

      # Stdout.line ("Part 2: \(sum3)")
      Stdout.line "Part 1: \(part1), Part 2: \(part2)"
      

    Task.attempt task \result ->
      when result is
         Ok {} -> Stdout.line ""
         Err err ->
                msg =
                    when err is
                        FileWriteErr _ PermissionDenied -> "PermissionDenied"
                        FileWriteErr _ Unsupported -> "Unsupported"
                        FileWriteErr _ (Unrecognized _ other) -> other
                        FileReadErr _ _ -> "Error reading file"
                        _ -> "Uh oh, there was an error!"

                {} <- Stderr.line msg |> Task.await
                Process.exit 1

