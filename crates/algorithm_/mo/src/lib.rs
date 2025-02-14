/// Mo's algorithm の実装に必要な操作を定義するトレイト
///
/// # 概要
/// - 区間クエリを効率的に処理するためのアルゴリズムを実装
/// - クエリを適切な順序で処理することで計算量を削減
///
/// # トレイト要件
/// - `Output`: クエリの結果型（`Default + Clone` を実装する必要あり）
///
/// # 実装方法
/// 以下のメソッドのうち、必要なものを実装する：
///
/// - 基本操作（どちらかを選択）
///   - `add`/`remove`: 方向に依存しない場合
///   - `add_left`/`add_right`/`remove_left`/`remove_right`: 方向に依存する場合
/// - `query`: 現在の状態での結果を計算
/// - `initial_position`: 必要に応じて初期位置を設定（デフォルトは`(0, 0)`）
///
/// # 計算量
/// - 移動1回あたりの計算量を O(α) として、全体で O(αN√Q)
///   - N: 配列の長さ
///   - Q: クエリの数
pub trait Mo {
    type Output: Default + Clone;

    /// 要素を追加する
    ///
    /// # 引数
    /// - `i`: 追加する要素のインデックス
    ///
    /// # 注意
    /// 左右の方向に依存する場合は、代わりに `add_left`/`add_right` を実装すること
    #[allow(unused_variables)]
    fn add(&mut self, i: usize) {
        unimplemented!()
    }

    /// 要素を削除する
    ///
    /// # 引数
    /// - `i`: 削除する要素のインデックス
    ///
    /// # 注意
    /// 左右の方向に依存する場合は、代わりに `remove_left`/`remove_right` を実装すること
    #[allow(unused_variables)]
    fn remove(&mut self, i: usize) {
        unimplemented!()
    }

    /// 左方向に要素を追加する
    ///
    /// # 動作
    /// - 要素 `a[i]` を追加
    /// - または点を `(i+1, j)` から `(i, j)` に移動
    fn add_left(&mut self, i: usize) {
        self.add(i);
    }

    /// 右方向に要素を追加する
    ///
    /// # 動作
    /// - 要素 `a[i]` を追加
    /// - または点を `(j, i)` から `(j, i+1)` に移動
    fn add_right(&mut self, i: usize) {
        self.add(i);
    }

    /// 左方向に要素を削除する
    ///
    /// # 動作
    /// - 要素 `a[i]` を削除
    /// - または点を `(i, j)` から `(i+1, j)` に移動
    fn remove_left(&mut self, i: usize) {
        self.remove(i);
    }

    /// 右方向に要素を削除する
    ///
    /// # 動作
    /// - 要素 `a[i]` を削除
    /// - または点を `(j, i+1)` から `(j, i)` に移動
    fn remove_right(&mut self, i: usize) {
        self.remove(i);
    }

    /// 現在の区間に対するクエリを実行する
    ///
    /// # 戻り値
    /// 現在の状態における計算結果
    fn query(&self) -> Self::Output;

    /// 初期位置を設定する
    ///
    /// # 戻り値
    /// - デフォルトでは `(0, 0)` を返す
    /// - 必要に応じてオーバーライドすること
    fn initial_position(&self) -> (usize, usize) {
        (0, 0)
    }

    /// すべてのクエリを処理する
    ///
    /// # 引数
    /// - `qs`: `(left, right)` の形式のクエリ配列
    ///
    /// # 計算量
    /// - 移動1回あたりの計算量を O(α) として O(αN√Q)
    ///   - N: 配列の長さ
    ///   - Q: クエリの数
    fn solve(&mut self, qs: &[(usize, usize)]) -> Vec<Self::Output> {
        let n = qs.iter().map(|&(l, r)| l.max(r)).max().unwrap();
        let q = qs.len();
        let w = 1.max((n as f64 / 1.0f64.max((q as f64 * 2.0 / 3.0).sqrt())).round() as usize);
        let mut ord = (0..q).collect::<Vec<_>>();
        ord.sort_unstable_by_key(|&i| {
            let (l, r) = qs[i];
            (l / w, if (l / w) & 1 == 0 { r } else { !r })
        });
        let (mut l, mut r) = self.initial_position();
        let mut res = vec![Default::default(); q];
        for i in ord {
            let (ll, rr) = qs[i];
            while l > ll {
                l -= 1;
                self.add_left(l);
            }
            while r < rr {
                self.add_right(r);
                r += 1;
            }
            while l < ll {
                self.remove_left(l);
                l += 1;
            }
            while r > rr {
                r -= 1;
                self.remove_right(r);
            }
            res[i] = self.query();
        }
        res
    }
}
