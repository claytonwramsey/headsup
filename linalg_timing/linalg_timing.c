
#include <assert.h>
#include <math.h>
#include <stddef.h>

/** The number of microphones. */
#define N_MICROPHONES 8

/** The speed of sound, in meters per second. */
#define SPEED_OF_SOUND 343.0f

/** A length in meters. */
typedef float Meters;

/** A time in seconds. */
typedef float Seconds;

/** A point in 3-dimensional space. */
typedef struct
{
    /** The x position. */
    Meters x;
    /** The y position. */
    Meters y;
    /** The z position. */
    Meters z;
} Point3d;

/** A point in space and time where a gunshot came from. */
typedef struct
{
    /** The position where the gunshot occurred. */
    Point3d position;
    /** The time at which the gunshot occurred. */
    Seconds t;
} ShotPoint;

/**
 * Attempt to find the source of a shot.
 *
 * Inputs:
 * - times: array containing times of sound receipt for each microphone (of length N_MICROPHONES)
 * - err_tolerance: maximum allowable error for this solver to consider itself converged
 * - step_scale_time: scale on gradient steps for this solver in time
 * - step_scale_space: scale on gradient steps for this solver in space
 * - n_iters: number of iterations for the solver to take
 * - out: place to store result in case of success
 *
 * Returns 0 on success and -1 if we cannot find a satisfying result.
 */
int find_shot_source(
    const Seconds *const times,
    const Point3d *const positions,
    const float err_tolerance,
    const float step_scale_time,
    const float step_scale_space,
    const unsigned int n_iters,
    ShotPoint *out);

/**
 * Use `prediction` and `positions` to compute the expected times of arrival for a given predicted
 * shot origin, and store the result in `out`.
 * `out` must point to a buffer of length `N_MICROPHONES`.
 */
static void predict_times(
    const ShotPoint *prediction,
    const Point3d *const positions,
    Seconds out[]);

/** Compute the distance between two points in 3D space. */
static float point3d_dist(const ShotPoint *x, const ShotPoint *y);

/** Compute the relative errors between predicted and expected. */
static void relative_time_errs(
    const Seconds *const expected,
    const Seconds *const predicted,
    const Seconds out[]);

/** Compute the mean of the sum of the squares of an array of `n` floats. */
static float mean_sum_square(const float *nums, const unsigned int n);

int find_shot_source(
    const Seconds *const times,
    const Point3d *const positions,
    const float err_tolerance,
    const float step_scale_time,
    const float step_scale_space,
    const unsigned int n_iters,
    ShotPoint *out)
{
    assert(err_tolerance > 0);
    assert(step_scale > 0);
    assert(times != NULL);
    assert(positions != NULL);

    // The predicted location of the source of the gunshot.
    ShotPoint prediction = {0};

    // set starting time to be the minimum of all times
    prediction.t = INFINITY;
    for (unsigned int i = 0; i < N_MICROPHONES; i++)
        prediction.t = fmin(prediction.t, times[i]);

    Seconds predicted_times[N_MICROPHONES];
    predict_times(&prediction, positions, &predicted_times);

    Seconds time_errs[N_MICROPHONES];
    relative_time_errs(times, predicted_times, &time_errs);

    Seconds mse = mean_sum_square(time_errs, N_MICROPHONES);
    for (unsigned int i = 0; i < n_iters && err_tolerance < mse; i++)
    {
        for (unsigned int mic = 0; mic < N_MICROPHONES; mic++)
        {
            prediction.t -= -step_scale_time * 2.0 * time_errs[mic] / N_MICROPHONES;

            prediction.position.x -= -step_scale_space * 2.0 *
        }
    }
}

static void predict_times(
    const ShotPoint *prediction,
    const Point3d *const positions,
    float *out)
{
    for (unsigned int i = 0; i < N_MICROPHONES; i++)
    {
        out[i] = prediction->t + point3d_dist(&prediction->position, &positions[i]) / SPEED_OF_SOUND;
    }
}

static float mean_sum_square(const float *nums, const unsigned int n)
{
    float avg = 0;
    for (int i = 0; i < nums; i++)
    {
        avg += nums[i] * nums[i];
    }
    return avg / n;
}