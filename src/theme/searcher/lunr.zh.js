(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    // AMD. Register as an anonymous module.
    define(factory)
  } else if (typeof exports === 'object') {
    /**
     * Node. Does not work with strict CommonJS, but
     * only CommonJS-like environments that support module.exports,
     * like Node.
     */
    module.exports = factory()
  } else {
    // Browser globals (root is window)
    factory()(root.lunr);
  }
}(this, function () {
  return function (lunr) {
    if ('undefined' === typeof lunr) {
      throw new Error('Lunr is not present. Please include / require Lunr before this script.');
    }
    
    /* register specific locale function */
    lunr.zh = function () {
      this.pipeline.reset();
      this.pipeline.add(
        lunr.zh.trimmer,
        lunr.zh.stopWordFilter,
        lunr.zh.stemmer
      );

      // for lunr version 2
      // this is necessary so that every searched word is also stemmed before
      // in lunr <= 1 this is not needed, as it is done using the normal pipeline
      if (this.searchPipeline) {
        this.searchPipeline.reset();
        this.searchPipeline.add(lunr.zh.stemmer)
      }
    };

    lunr.zh.tokenizer = function (str) {
      if (!arguments.length || str === null || str === undefined) return [];
      if (Array.isArray(str)) {
        var arr = str.filter(function (token) {
          if (token === null || token === undefined) {
            return false;
          }

          return true;
        });

        arr = arr.map(function (t) {
          return lunr.utils.toString(t);
        });

        var out = [];
        arr.forEach(function (item) {
          var tokens = item.split(lunr.tokenizer.seperator);
          out = out.concat(tokens);
        }, this);

        return out;
      }

      return str.toString().trim().split(lunr.tokenizer.seperator);
    };


    /* lunr trimmer function */
    lunr.zh.trimmer = function (_token) {
      return _token;
    }

    lunr.Pipeline.registerFunction(lunr.zh.trimmer, 'trimmer-zh');

    /* lunr stemmer function */
    lunr.zh.stemmer = (function () {
      /* and return a function that stems a word for the current locale */
      return function (token) {
        return token;
      }
    })();
    lunr.Pipeline.registerFunction(lunr.zh.stemmer, 'stemmer-zh');

    lunr.zh.stopWordFilter = function (token) {
      return token;
    };
    lunr.Pipeline.registerFunction(lunr.zh.stopWordFilter, 'stopWordFilter-zh');
  };
}))