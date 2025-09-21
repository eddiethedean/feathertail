TinyFrame
=========

The main DataFrame class in feathertail.

.. autoclass:: feathertail.TinyFrame
   :members:
   :undoc-members:
   :show-inheritance:

Constructor Methods
-------------------

.. automethod:: feathertail.TinyFrame.__init__

.. automethod:: feathertail.TinyFrame.from_dicts

.. automethod:: feathertail.TinyFrame.from_pandas

Data Access
-----------

.. automethod:: feathertail.TinyFrame.to_dicts

.. automethod:: feathertail.TinyFrame.to_pandas

.. automethod:: feathertail.TinyFrame.head

.. automethod:: feathertail.TinyFrame.tail

.. automethod:: feathertail.TinyFrame.info

Basic Operations
----------------

.. automethod:: feathertail.TinyFrame.len

.. automethod:: feathertail.TinyFrame.shape

.. automethod:: feathertail.TinyFrame.columns

.. automethod:: feathertail.TinyFrame.dtypes

Data Manipulation
-----------------

.. automethod:: feathertail.TinyFrame.filter

.. automethod:: feathertail.TinyFrame.sort_values

.. automethod:: feathertail.TinyFrame.dropna

.. automethod:: feathertail.TinyFrame.fillna

.. automethod:: feathertail.TinyFrame.cast_column

GroupBy Operations
------------------

.. automethod:: feathertail.TinyFrame.groupby

Join Operations
---------------

.. automethod:: feathertail.TinyFrame.inner_join

.. automethod:: feathertail.TinyFrame.left_join

.. automethod:: feathertail.TinyFrame.right_join

.. automethod:: feathertail.TinyFrame.outer_join

.. automethod:: feathertail.TinyFrame.cross_join

Analytics Functions
-------------------

.. automethod:: feathertail.TinyFrame.describe

.. automethod:: feathertail.TinyFrame.corr

.. automethod:: feathertail.TinyFrame.corr_with

.. automethod:: feathertail.TinyFrame.cov

.. automethod:: feathertail.TinyFrame.cov_with

.. automethod:: feathertail.TinyFrame.skew

.. automethod:: feathertail.TinyFrame.kurtosis

.. automethod:: feathertail.TinyFrame.quantile

.. automethod:: feathertail.TinyFrame.mode

.. automethod:: feathertail.TinyFrame.nunique

Time Series Operations
----------------------

.. automethod:: feathertail.TinyFrame.to_timestamps

.. automethod:: feathertail.TinyFrame.dt_year

.. automethod:: feathertail.TinyFrame.dt_month

.. automethod:: feathertail.TinyFrame.dt_day

.. automethod:: feathertail.TinyFrame.dt_hour

.. automethod:: feathertail.TinyFrame.dt_minute

.. automethod:: feathertail.TinyFrame.dt_second

.. automethod:: feathertail.TinyFrame.dt_day_of_week

.. automethod:: feathertail.TinyFrame.dt_day_of_year

.. automethod:: feathertail.TinyFrame.dt_diff

.. automethod:: feathertail.TinyFrame.dt_shift

Window Functions
----------------

.. automethod:: feathertail.TinyFrame.rolling_mean

.. automethod:: feathertail.TinyFrame.rolling_sum

.. automethod:: feathertail.TinyFrame.rolling_std

.. automethod:: feathertail.TinyFrame.expanding_mean

.. automethod:: feathertail.TinyFrame.expanding_sum

Ranking Functions
-----------------

.. automethod:: feathertail.TinyFrame.rank

.. automethod:: feathertail.TinyFrame.pct_change

String Operations
-----------------

.. automethod:: feathertail.TinyFrame.str_upper

.. automethod:: feathertail.TinyFrame.str_lower

.. automethod:: feathertail.TinyFrame.str_strip

.. automethod:: feathertail.TinyFrame.str_replace

.. automethod:: feathertail.TinyFrame.str_split

.. automethod:: feathertail.TinyFrame.str_contains

.. automethod:: feathertail.TinyFrame.str_len

.. automethod:: feathertail.TinyFrame.str_cat

Data Validation
---------------

.. automethod:: feathertail.TinyFrame.validate_not_null

.. automethod:: feathertail.TinyFrame.validate_range

.. automethod:: feathertail.TinyFrame.validate_pattern

.. automethod:: feathertail.TinyFrame.validate_unique

.. automethod:: feathertail.TinyFrame.validation_summary
