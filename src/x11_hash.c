#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>

#include "x11/Blake.h"
#include "x11/Bmw.h"
#include "x11/CubeHash.h"
#include "x11/Echo.h"
#include "x11/Groestl.h"
#include "x11/Jh.h"
#include "x11/Keccak.h"
#include "x11/Luffa.h"
#include "x11/Shavite.h"
#include "x11/Simd.h"
#include "x11/Skein.h"
#include "x11/sph_types.h"

void x11_hash(const char* input, char* output) {
    uint32_t hashA[16], hashB[16];

    sph_blake_big_context ctx_blake;
    sph_blake512_init(&ctx_blake);
    sph_blake512(&ctx_blake, input, 80);
    sph_blake512_close(&ctx_blake, hashA);

    sph_bmw_big_context ctx_bmw;
    sph_bmw512_init(&ctx_bmw);
    sph_bmw512(&ctx_bmw, hashA, 64);
    sph_bmw512_close(&ctx_bmw, hashB);

    sph_groestl_big_context ctx_groestl;
    sph_groestl512_init(&ctx_groestl);
    sph_groestl512(&ctx_groestl, hashB, 64);
    sph_groestl512_close(&ctx_groestl, hashA);

    sph_skein_big_context ctx_skein;
    sph_skein512_init(&ctx_skein);
    sph_skein512(&ctx_skein, hashA, 64);
    sph_skein512_close(&ctx_skein, hashB);

    sph_jh_context ctx_jh;
    sph_jh512_init(&ctx_jh);
    sph_jh512(&ctx_jh, hashB, 64);
    sph_jh512_close(&ctx_jh, hashA);

    sph_keccak_context ctx_keccak;
    sph_keccak512_init(&ctx_keccak);
    sph_keccak512(&ctx_keccak, hashA, 64);
    sph_keccak512_close(&ctx_keccak, hashB);

    sph_luffa512_context ctx_luffa;
    sph_luffa512_init(&ctx_luffa);
    sph_luffa512(&ctx_luffa, hashB, 64);
    sph_luffa512_close(&ctx_luffa, hashA);

    sph_cubehash_context ctx_cubehash;
    sph_cubehash512_init(&ctx_cubehash);
    sph_cubehash512(&ctx_cubehash, hashA, 64);
    sph_cubehash512_close(&ctx_cubehash, hashB);

    sph_shavite_big_context ctx_shavite;
    sph_shavite512_init(&ctx_shavite);
    sph_shavite512(&ctx_shavite, hashB, 64);
    sph_shavite512_close(&ctx_shavite, hashA);

    sph_simd_big_context ctx_simd;
    sph_simd512_init(&ctx_simd);
    sph_simd512(&ctx_simd, hashA, 64);
    sph_simd512_close(&ctx_simd, hashB);

    sph_echo_big_context ctx_echo;
    sph_echo512_init(&ctx_echo);
    sph_echo512(&ctx_echo, hashB, 64);
    sph_echo512_close(&ctx_echo, hashA);
    memcpy(output, hashA, 32);
}
