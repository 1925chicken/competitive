#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
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
void solve(){
}
//
// Created by chicken1925 on 2021/01/22.
//

template<typename T>
class LCA {
public:
    vector<int_fast32_t> depth;
    const int_fast32_t dig;
    vector<vector<int_fast32_t>> g;
    vector<vector<int_fast32_t>> cost;
    vector<int_fast32_t> costs;
    int_fast32_t n;
    vector<vector<int_fast32_t>> doubling;

    LCA(int_fast64_t _n) : n(_n),dig(64){
        doubling.assign(dig, vector<int_fast32_t>(n, -1));
        costs.resize(n);
        depth.resize(n);
        g.resize(n);
        cost.resize(n);
    }

    void addedge(int_fast32_t u, int_fast32_t v, int_fast32_t c = 0) {
        g[u].emplace_back(v);
        cost[u].emplace_back(c);
        g[v].emplace_back(u);
        cost[v].emplace_back(c);

    }

    void dfs(int_fast32_t now, int_fast32_t par = -1, int_fast32_t d = 0, int_fast32_t c = 0) {
        doubling[0][now] = par;
        depth[now] = d;
        costs[now] = c;
        rep(i, len(g[now])) {
            if (g[now][i] != par)dfs(g[now][i], now, d + 1, c + cost[now][i]);
        }
    }

    void construct() {
        dfs(0);
        for (int_fast32_t i = 0; i < dig - 1; i++) {
            for (int_fast32_t j = 0; j < doubling[i].size(); j++) {
                if (doubling[i][j] == -1)doubling[i + 1][j] = -1;
                else doubling[i + 1][j] = doubling[i][doubling[i][j]];
            }
        }
    }

    int_fast32_t query_answer(int_fast32_t u, int_fast32_t v) {
        if (depth[u] > depth[v])swap(u, v);
        for (int_fast32_t i = dig - 1; 0 <= i; i--) {
            if (((depth[v] - depth[u]) >> i) & 1) v = doubling[i][v];
        }
        if (u == v) return u;
        for (int_fast32_t i = dig - 1; 0 <= i; --i) {
            if (doubling[i][u] != doubling[i][v]) {
                u = doubling[i][u];
                v = doubling[i][v];
            }
        }
        return doubling[0][u];
    }

    int_fast32_t length(int_fast32_t u, int_fast32_t v) {
        return depth[u] + depth[v] - 2 * depth[query_answer(u, v)];
    }

    int_fast32_t dist(int_fast32_t u, int_fast32_t v) {
        return costs[u] + costs[v] - 2 * costs[query_answer(u, v)];
    }

    bool isOnPath(int_fast32_t u, int_fast32_t v, int_fast32_t x) {
        return length(u, x) + length(v, x) == length(u, v);
    }
};
int main(){
    ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    i64 n,m;
    cin >> n >> m;
    LCA<i64> lca(n);
    rep(i,n - 1) {
        i64 a,b;
        cin >> a >> b;
        a--,b--;
        lca.addedge(a,b,1);
    }
    lca.construct();
    rep(i,m) {
        i64 a,b;
        cin >> a >> b;
        a--,b--;
        if(lca.dist(a,b) % 2) {
            cout << "Road" << endl;
        }else {
            cout << "Town" << endl;
        }
    }
}
