#pragma GCC target("avx2")
#pragma GCC optimize("Ofast")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<queue>
#include<algorithm>
#include<vector>
#include<set>
#include<map>
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

void warshall_froyd(std::vector<std::vector<int_fast32_t>> dist,i32 &ans){//隣接行列を使う
    i32 n = len(dist);
   // vec<vec<i64>> d = vec<vec<i32>>(n,vec<i32>(n));
    for(int_fast32_t k = 0; k < n; k++){
        for(int_fast32_t i = 0; i < n; i++){
            for(int_fast32_t j = 0; j < n; j++){
                dist[i][j] = min(dist[i][j],dist[i][k] + dist[k][j]);
                if (dist[i][j] < 1e9) {
                    ans += dist[i][j];
                }
            }
        }
    }
}
bool negative_cycle(std::vector<std::vector<int_fast64_t>> &dist){
    i64 n = dist.size();
    for(int_fast32_t i = 0; i < n; i++){
        if(dist[i][i] < 0)return true;
    }
    return false;
}
int main(){
    ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    i32 n,m;
    cin >> n >> m;
    i32 ans = 0;
    vec<vec<i32>> dist = vec<vec<i32>> (n,vec<i32>(n,1e9));
    rep(i,m) {
        i32 a,b,c;
        cin >> a >> b >> c;
        a--,b--;
        dist[a][b] = c;
    }
    rep(i,n) {
        dist[i][i] = 0;
    }
    warshall_froyd(dist,ans);
    cout << ans << endl;
}
