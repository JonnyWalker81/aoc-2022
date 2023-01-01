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


Shape : [Rock, Paper, Scissors]
Round : {opp: Shape, ours: Shape}

main : Task {} []
main =
    path = Path.fromStr "sample.txt"
    # path = Path.fromStr "input.txt"
    task =
      contents <- File.readUtf8 path |> Task.await
      lines = Str.split contents "\n"
      joined = Str.joinWith lines "--"
      # Stdout.line "\(joined)" 
      rounds = List.map lines (\l ->
         shapes = Str.split l " "
         opp = List.get shapes 0 |> Result.withDefault ""
         ours = List.get shapes 1 |> Result.withDefault ""
         {opp: opp, ours: ours})
      len = List.len rounds |> Num.toStr
      Stdout.line  len

      
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

