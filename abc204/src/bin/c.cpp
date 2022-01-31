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
vec<vec<i64>> flag;
vec<vec<bool>> seen;
void solve(vec<vec<i64>> &g,i64 from,i64 now){
   // cout << from << " "  << now << endl;
    seen[from][now] = true;
    g[from][now] = 1;
    for (auto v:g[now]) {
        if (!seen[from][v])
        solve(g,from,v);
    }
}
int main(){
    ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    i64 n,m;
    cin >> n >> m;
    vec<vec<i64>> g(n);
    rep(i,m){
        i64 a,b;
        cin >> a >> b;
        a--,b--;
        g[a].emplace_back(b);
    }


    flag.resize(n,vec<i64>(n));
    seen.resize(n,vec<bool>(n,false));
    rep(i,n){
        flag[i][i] = 1;
        seen[i][i] = true;
    }
    rep(i,n){
        solve(g,i,i);
    }
    i64 ans = 0;
    rep(i,n){
        rep(j,n){
            ans += flag[i][j];
        }
    }
    rep(i,n){
        rep(j,n) {
           cout << flag[i][j] << " \n"[j == n - 1];
        }
    }
    cout << ans << endl;
}
