export class Result<T, E extends Error> {
	#ok: T | null;
	#err: E | null;

	constructor(ok: T | null, err: E | null) {
		if (ok == null && err == null) {
			throw new Error('Both ok and err cannot be null');
		}

		if (ok != null && err != null) {
			throw new Error('Both ok and err cannot be non null');
		}

		this.#ok = ok;
		this.#err = err;
	}

	static Error<E extends Error>(err: E) {
		return new Result(null, err);
	}

	static Success<T>(result: T) {
		return new Result(result, null);
	}

	isOk(): boolean {
		return this.#ok != null;
	}

	isErr(): boolean {
		return this.err != null;
	}

	err(): E {
		if (this.#err == null) {
			throw new Error('Error is null');
		}
		return this.#err;
	}

	unwrap(): T {
		if (this.#ok == null) {
			throw new Error('result is null');
		}
		return this.#ok;
	}
}
