#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<set>
#include<stack>
#include<vector>
#if __has_include(<atcoder/all>)
#include<atcoder/all>
using namespace atcoder;
#endif
//#include<boost/multiprecision/cpp_int.hpp>
//namespace mp = boost::multiprecision;
using namespace std;
using i32 = int_fast32_t;
using i64 = int_fast64_t;
using usize = size_t;
using u32 = uint_fast32_t;
using u64 = uint_fast64_t;
using i128 = __int128_t;
using u128 = __uint128_t;
using ld = long double;
template<typename T>
using vec = vector<T>;
#define rep(i, n) for (i64 i = 0; i < (i64)(n); ++i)
#define all(a) (a).begin(),(a).end()
#define rall(a) (a).rbegin(),(a).rend()
#define len(hoge) (i64)((hoge).size())
using P = pair<i64,i64>;
template<class T, class S> ostream &operator<<(ostream &os, const pair<T,S> &p){
    os << p.first << " " << p.second;
    return os;
}
vector<int> topo_sort(const vec<vec<i64>> &G) {  // bfs
    vector<int> ans;
    int n = (int)G.size();
    vector<int> ind(n);            // ind[i]: 頂点iに入る辺の数(次数)
    for (int i = 0; i < n; i++) {  // 次数を数えておく
        for (auto e : G[i]) {
            ind[e]++;
        }
    }
    queue<int> que;
    for (int i = 0; i < n; i++) {  // 次数が0の点をキューに入れる
        if (ind[i] == 0) {
            que.push(i);
        }
    }
    while (!que.empty()) {  // 幅優先探索
        int now = que.front();
        ans.push_back(now);
        que.pop();
        for (auto e : G[now]) {
            ind[e]--;
            if (ind[e] == 0) {
                que.push(e);
            }
        }
    }
    return ans;
}

void solve(){
    i64 n,m;
    cin >> n >> m;
    vec<vec<i64>> f = vec<vec<i64>>(m,vec<i64>());
    vec<vec<i64>> g = vec<vec<i64>>(n,vec<i64>());
    rep(i,m) {
        i64 k;
        cin >> k;
        rep(j,k) {
            i64 a;
            cin >> a;
            a -= 1;
            f[i].push_back(a);
        }
    }
    rep(i,m) {
        for(i64 j = 1; j < len(f[i]); j += 1) {
            g[f[i][j - 1]].push_back(f[i][j]);
        }
    }
    auto v = topo_sort(g);
    if(n !=len(v)) cout << "No" << endl;
    else cout << "Yes" << endl;
}
int main(){
    ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    solve();
}
