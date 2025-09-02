# VieShare Axum API Documentation

## Overview
REST API for VieShare e-commerce platform built with Axum, compatible with PocketBase schema.

## API Endpoints

### Authentication
- POST `/api/collections/users/auth-with-password` - Login
- POST `/api/collections/users/records` - Register
- POST `/api/collections/users/request-password-reset` - Password reset

### Categories
- GET `/api/collections/categories/records` - List categories
- GET `/api/collections/categories/records/{id}` - Get category
- POST `/api/collections/categories/records` - Create category
- PATCH `/api/collections/categories/records/{id}` - Update category
- DELETE `/api/collections/categories/records/{id}` - Delete category

### Products
- GET `/api/collections/products/records` - List products
- GET `/api/collections/products/records/{id}` - Get product
- POST `/api/collections/products/records` - Create product
- PATCH `/api/collections/products/records/{id}` - Update product
- DELETE `/api/collections/products/records/{id}` - Delete product

### Stores
- GET `/api/collections/stores/records` - List stores
- GET `/api/collections/stores/records/{id}` - Get store
- POST `/api/collections/stores/records` - Create store
- PATCH `/api/collections/stores/records/{id}` - Update store
- DELETE `/api/collections/stores/records/{id}` - Delete store

### Carts & Orders
- GET `/api/collections/carts/records` - List carts
- GET `/api/collections/cart_items/records` - List cart items
- GET `/api/collections/orders/records` - List orders
- POST `/api/collections/orders/records` - Create order

## Authentication
All endpoints require Bearer token authentication except login/register.

```
Authorization: Bearer <jwt_token>
```