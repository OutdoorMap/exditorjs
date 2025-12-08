# test/test_helper.exs

ExUnit.start()
{:ok, _} = Application.ensure_all_started(:exditorjs)