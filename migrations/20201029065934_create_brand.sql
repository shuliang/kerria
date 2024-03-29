CREATE TABLE `brand` (
  `id` BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(128) NOT NULL DEFAULT '' COMMENT '品牌名称',
  `sequence` INT NOT NULL DEFAULT 0 COMMENT '品牌顺序',
  `status` TINYINT NOT NULL DEFAULT 0 COMMENT '状态，0：默认，1：已删除',
  `creator` VARCHAR(32) NOT NULL DEFAULT '',
  `modifier` VARCHAR(32) NOT NULL DEFAULT '',
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  KEY `idx_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='商品品牌表';
