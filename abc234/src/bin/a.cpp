#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<cstddef>
#include<vector>
#include<algorithm>
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
constexpr i64 MOD = 998244353;


template<i64 modulus>
class modcal {
public:
	i64 a;

	constexpr modcal(const i64 x = 0) noexcept: a(x % modulus) {}

	constexpr i64 &value() noexcept { return a; }

	constexpr const i64 &value() const noexcept { return a; }

	constexpr modcal operator+(const modcal rhs) const noexcept {
		return modcal(*this) += rhs;
	}

	constexpr modcal operator-(const modcal rhs) const noexcept {
		return modcal(*this) -= rhs;
	}

	constexpr modcal operator*(const modcal rhs) const noexcept {
		return modcal(*this) *= rhs;
	}

	constexpr modcal operator/(const modcal rhs) noexcept {
		return modcal(*this) /= rhs;
	}

	constexpr modcal &operator+=(const modcal rhs) noexcept {
		a += rhs.a;
		if (a >= modulus) {
			a -= modulus;
		}
		return *this;
	}

	constexpr modcal &operator-=(const modcal rhs) noexcept {
		if (a < rhs.a) {
			a += modulus;
		}
		a -= rhs.a;
		return *this;
	}

	constexpr modcal &operator*=(const modcal rhs) noexcept {
		a = a * rhs.a % modulus;
		return *this;
	}
	/*ostream &operator<<(ostream &os, const modcal<MOD> I){
		os << I.a;
		return os;
	}*/
	constexpr modcal &operator/=(modcal rhs) noexcept {
		i64 exp = modulus - 2;
		while (exp) {
			if (exp % 2) {
				*this *= rhs;
			}
			rhs *= rhs;
			exp /= 2;
		}
		return *this;
	}

	constexpr modcal<MOD> modpow(const modcal<MOD> &a, i64 n) {
		if (n == 0)
			return 1;
		auto t = modpow(a, n / 2);
		t = t * t;
		if (n & 1)
			t = t * a;
		return t;
	}

};

using modc = modcal<MOD>;
class com {
	vector <modc> fac, inv;
	i64 n;
public:
	com(i64 N) : fac(N + 1), inv(N + 1), n(N) {
		fac[0] = 1;
		for (i64 i = 1; i <= n; i++)fac[i] = fac[i - 1] * i;
		inv[n] = fac[n].modpow(fac[n], MOD - 2);
		for (i64 i = n; i >= 1; i--)inv[i - 1] = inv[i] * i;

	}

	modc combination(i64 n, i64 k) {
		if (k < 0 || k > n)return 0;
		return fac[n] * inv[k] * inv[n - k];
	}

	modc permutation(i64 n, i64 k) {
		if (k < 0 || k > n)return 0;
		return combination(n, k) * fac[k];
	}
};
istream &operator >> (istream &is,modc &x){
	i64 t;
	is >> t;
	x = t;
	return is;
}
ostream &operator << (ostream &os,const modc &x){
	os << x.a;
	return os;
}

void solve(){
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	modc ans = 0;
	modc fac = 1;
	string s;
	cin >> s;
	vec<i64> con(26,0);
	rep(i,len(s))con[s[i] - 'a']++;
	sort(all(con));
	rep(i,len(s)) {
		fac *= (i + 1);
	}
	modc divs = 1;
	rep(i,len(s)) {
		i64 t = con.end() - lower_bound(all(con),2);
		divs *= divs.modpow(i+1,t);
	}
	for(i64 i = len(s); 2 <= i; i--) {
		i64 t = con.end() - lower_bound(all(con),i);
		ans += fac / divs;
		fac /= (len(s) - i + 1);
		divs /= divs.modpow(i,t);
		divs *= divs.modpow(i - 1,t);
		cout << ans << endl;
	}
	ans += (con.end() - lower_bound(all(con),1));
	cout << ans << endl;
}
