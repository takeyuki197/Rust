#include "stdafx.h"
#include "SumAll.h"

double sum_all(long inValNum, const double* inVals)
{
	double ret = 0;
	for (long i = 0; i<inValNum; ++i) {
		ret += inVals[i];
	}
	return ret;
}

double sum(double inA, double inB)
{
	return inA + inB;
}
