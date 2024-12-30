# 概要

ココフォリアの全ログ出力により入手できるHTMLファイルから、成功・失敗・クリティカル・ファンブルのログを抽出します。<br>
いわゆるオレオレツールなので、出力などを自分で調整することをお勧めします。<br>
rustの勉強ついでに作ったものなので一切を保証しません。<br>

# 配布

exeの配布はしません。自力でクローンしてcargoでrunないしbuildしてください。<br>
~~TRPGerかつRustaceanの集合を満たす人がどれだけいるのか~~


# 使い方

引数にパスを渡す、またはexeファイルにHTMLファイルをドラッグアンドドロップすると、コンソールに抽出結果が出力されます。<br>
「---start---」というチャットがある場合、それ以前を無視します。シナリオ開始前の試し振りなどを無視できます。<br>
log_summary.print_logメソッドを使うことで、具体的な技能や成功値を出力できます。<br>
出力の調整だけならmain.rsとlog_summary.rsをいじるだけでいいと思います。たぶん。<br>