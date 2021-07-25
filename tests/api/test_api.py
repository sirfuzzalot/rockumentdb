"""
Test suite for REST API
"""

import httpx
import json
import pytest
import subprocess


BASE_URL = "http://127.0.0.1:8000/api/v2/test"


@pytest.fixture(scope="module")
def server():
    """
    Spins up the Database server
    """
    proc = subprocess.Popen(["cargo", "run"])

    yield

    proc.terminate()


def test_insert_one_doc(server):
    response = httpx.post(
        BASE_URL,
        json=[
            {
                "username": "johnperry",
                "email": "johnperry@example.com",
                "first_name": "John",
                "last_name": "Perry",
                "age": 75
            },
            {
                "username": "louiswu",
                "email": "louiswu@example.com",
                "first_name": "Louis",
                "last_name": "Wu",
                "age": 200
            }
        ]
    )
    response.raise_for_status()
    assert response.json() == [1,2]


def test_find_one_field(server):
    response = httpx.get(f'{BASE_URL}?query={{username:"johnperry"}}')
    response.raise_for_status()
    assert json.dumps(response.json(), sort_keys=True) == json.dumps(
        [{
            "username": "johnperry",
            "email": "johnperry@example.com",
            "first_name": "John",
            "last_name": "Perry",
            "age": 75,
        }],
        sort_keys=True,
    )

def test_find_two_fields(server):
    response = httpx.get(f'{BASE_URL}?query={{username:"johnperry",age:75}}')
    response.raise_for_status()
    assert json.dumps(response.json(), sort_keys=True) == json.dumps(
        [{
            "username": "johnperry",
            "email": "johnperry@example.com",
            "first_name": "John",
            "last_name": "Perry",
            "age": 75,
        }],
        sort_keys=True,
    )

