-- Add migration script here
DROP TABLE IF EXISTS `users`;

CREATE TABLE IF NOT EXISTS `domains`(
  `id` varchar(100) NOT NULL,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `default_role_id` INTEGER,
  `admin_role_id` INTEGER,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW()
);

DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
  `id` varchar(100) NOT NULL,
  `username` varchar(50) NOT NULL,
  `password` varchar(100) NOT NULL,
  `email` varchar(200) DEFAULT NULL,
  `last_logined_at` datetime NOT NULL,
  `created_at` datetime NOT NULL,
  `avatar` TEXT,
  `memo` TEXT,
  `sys_role` VARCHAR(50) NOT NULL DEFAULT 'common',
  `is_actived` BOOLEAN NOT NULL DEFAULT true,
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `roles`(
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `value` VARCHAR(200) NOT NULL,
  `level` INTEGER NOT NULL,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `user_has_roles`(
  `user_id` VARCHAR(100) REFERENCES `users`(`id`),
  `role_id` INTEGER REFERENCES `roles`(`id`),
  `expire` TIMESTAMP NOT NULL,
  PRIMARY KEY(`user_id`, `role_id`)
);

CREATE TABLE IF NOT EXISTS `actions`(
  `id` INT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `value` TEXT,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `role_has_actions`(
  `action_id` INTEGER REFERENCES `actions`(`id`),
  `role_id` INTEGER REFERENCES `roles`(`id`),
  PRIMARY KEY(`action_id`, `role_id`)
);

CREATE TABLE IF NOT EXISTS `organizations` (
  `id` varchar(100) NOT NULL,
  `name` VARCHAR(100) NOT NULL,
  `description` TEXT,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `domain_id` VARCHAR(100) NOT NULL REFERENCES `domains`(`id`),
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `updated_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `created_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  `updated_by` VARCHAR(100) NOT NULL REFERENCES `users`(`id`),
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `user_has_organizations`(
  `user_id` VARCHAR(100) REFERENCES `users`(`id`),
  `org_id` VARCHAR(100) REFERENCES `organizations`(`id`),
  `expire` TIMESTAMP NOT NULL,
  PRIMARY KEY(`user_id`, `org_id`)
);

INSERT INTO `users`(`id`,`username`,`password`,`email`,`avatar`,`memo`,`sys_role`,`is_actived`,`last_logined_at`,`created_at`)
VALUES
('0c5b2b97-aefe-4110-80b5-fea91359f5b1','admin','$2b$04$BUPiWXysNDZw3ky8rQMyg.LsKyL80vGWgbwWzSUBLlfOFgqHz8jKq',NULL,NULL,NULL,'admin',TRUE,'2021-07-27 10:36:43.929291','2021-07-27 02:36:44.028063');