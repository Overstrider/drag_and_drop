FROM python:3.13-slim

# install system dependencies to compile Rust
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# copy requirements
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# copy and compile Rust parser
COPY rust_parser rust_parser/
WORKDIR /app/rust_parser
RUN maturin build --release
RUN pip install target/wheels/*.whl

# go back to the main directory
WORKDIR /app

# copy Python code
COPY . .

# create data directory
RUN mkdir -p /app/data

# expose port
EXPOSE 8000

# command to start the application
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"] 