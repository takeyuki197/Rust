#pragma once

#ifdef SUMALL_EXPORTS
#define SUMALL_API __declspec(dllexport)
#else
#define SUMALL_API __declspec(dllimport)
#endif

extern "C" SUMALL_API double sum_all(long inValNum, const double* inVals);

extern "C" SUMALL_API double sum(double inA, double inB);
