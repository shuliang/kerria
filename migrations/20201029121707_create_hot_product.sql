CREATE TABLE `hot_product` (
  `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `product_id` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '商品ID product.id',
  `status` TINYINT NOT NULL DEFAULT 0 COMMENT '状态，0：正常，1：已删除',
  `creator` VARCHAR(32) NOT NULL DEFAULT '',
  `modifier` VARCHAR(32) NOT NULL DEFAULT '',
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  KEY `idx_pid` (`product_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='热门商品表';
